
use std::fmt::Debug;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::cell::Cell;

use serde::{Serialize, Serializer};

pub type FcLineWriter = std::io::LineWriter<std::fs::File>;

/// Static instance used for handling metrics.
pub static METRICS: Metrics<FirecrackerMetrics, FcLineWriter> =
    Metrics::<FirecrackerMetrics, FcLineWriter>::new(FirecrackerMetrics::new());

/// Metrics system.
// All member fields have types which are Sync, and exhibit interior mutability, so
// we can call operations on metrics using a non-mut static global variable.
#[derive(Debug)]
pub struct Metrics<T: Serialize, M: Write + Send> {
    // Metrics will get flushed here.
    metrics_buf: OnceLock<Mutex<M>>,
    pub app_metrics: T,
}

impl<T: Serialize + Debug, M: Write + Send + Debug> Metrics<T, M> {
    /// Creates a new instance of the current metrics.
    // TODO: We need a better name than app_metrics (something that says that these are the actual
    // values that we are writing to the metrics_buf).
    pub const fn new(app_metrics: T) -> Metrics<T, M> {
        Metrics {
            metrics_buf: OnceLock::new(),
            app_metrics,
        }
    }

    /// Initialize metrics system (once and only once).
    /// Every call made after the first will have no effect besides returning `Ok` or `Err`.
    ///
    /// This function is supposed to be called only from a single thread, once.
    /// It is not thread-safe and is not meant to be used in a multithreaded
    /// scenario. The reason `is_initialized` is an `AtomicBool` instead of
    /// just a `bool` is that `lazy_static` enforces thread-safety on all its
    /// members.
    ///
    /// # Arguments
    ///
    /// * `metrics_dest` - Buffer for JSON formatted metrics. Needs to implement `Write` and `Send`.
    pub fn init(&self, metrics_dest: M) -> Result<(), MetricsError> {
        self.metrics_buf
            .set(Mutex::new(metrics_dest))
            .map_err(|_| MetricsError::AlreadyInitialized)
    }

    /// Writes metrics to the destination provided as argument upon initialization of the metrics.
    /// Upon failure, an error is returned if metrics system is initialized and metrics could not be
    /// written.
    /// Upon success, the function will return `True` (if metrics system was initialized and metrics
    /// were successfully written to disk) or `False` (if metrics system was not yet initialized).
    ///
    /// This function is usually supposed to be called only from a single thread and
    /// is not meant to be used in a multithreaded scenario. The reason
    /// `metrics_buf` is enclosed in a `Mutex` is that `lazy_static` enforces
    /// thread-safety on all its members.
    /// The only exception is for signal handlers that result in process exit, which may be run on
    /// any thread. To prevent the race condition present in the serialisation step of
    /// SharedIncMetrics, deadly signals use SharedStoreMetrics instead (which have a thread-safe
    /// serialise implementation).
    /// The only known caveat is that other metrics may not be properly written before exiting from
    /// a signal handler. We make this compromise since the process will be killed anyway and the
    /// important metric in this case is the signal one.
    /// The alternative is to hold a Mutex over the entire function call, but this increases the
    /// known deadlock potential.
    pub fn write(&self) -> Result<bool, MetricsError> {
        if let Some(lock) = self.metrics_buf.get() {
            match serde_json::to_string_pretty(&self.app_metrics) {
                Ok(msg) => {
                    if let Ok(mut guard) = lock.lock() {
                        // No need to explicitly call flush because the underlying LineWriter
                        // flushes automatically whenever a newline is
                        // detected (and we always end with a newline the
                        // current write).
                        guard
                            .write_all(format!("{msg}\n",).as_bytes())
                            .map_err(MetricsError::Write)
                            .map(|_| true)
                    } else {
                        // We have not incremented `missed_metrics_count` as there is no way to push
                        // metrics if destination lock got poisoned.
                        panic!(
                            "Failed to write to the provided metrics destination due to poisoned \
                             lock"
                        );
                    }
                }
                Err(err) => Err(MetricsError::Serde(err.to_string())),
            }
        } else {
            // If the metrics are not initialized, no error is thrown but we do let the user know
            // that metrics were not written.
            Ok(false)
        }
    }
    pub fn write_devmetrics(&self, perdevmetrics: String) -> Result<bool, MetricsError> {
        if let Some(lock) = self.metrics_buf.get() {
            if let Ok(mut guard) = lock.lock() {
                // No need to explicitly call flush because the underlying LineWriter
                // flushes automatically whenever a newline is
                // detected (and we always end with a newline the
                // current write).
                guard
                    .write_all(format!("{perdevmetrics}\n",).as_bytes())
                    .map_err(MetricsError::Write)
                    .map(|_| true)
            } else {
                // We have not incremented `missed_metrics_count` as there is no way to push
                // metrics if destination lock got poisoned.
                panic!(
                    "Failed to write to the provided metrics destination due to poisoned \
                        lock"
                );
            }
        } else {
            // If the metrics are not initialized, no error is thrown but we do let the user know
            // that metrics were not written.
            Ok(false)
        }
    }
}

impl<T: Serialize + Debug, M: Write + Send + Debug> Deref for Metrics<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.app_metrics
    }
}

/// Describes the errors which may occur while handling metrics scenarios.
#[derive(Debug, thiserror::Error)]
pub enum MetricsError {
    /// First attempt at initialization failed.
    #[allow(dead_code)]
    #[error("{0}")]
    NeverInitialized(String),
    /// The metrics system does not allow reinitialization.
    #[error("Reinitialization of metrics not allowed.")]
    AlreadyInitialized,
    /// Error in the serialization of metrics instance.
    #[error("{0}")]
    Serde(String),
    /// Writing the specified buffer failed.
    #[error("Failed to write metrics: {0}")]
    Write(std::io::Error),
}

/// Used for defining new types of metrics that act as a counter (i.e they are continuously updated
/// by incrementing their value).
pub trait IncMetric {
    /// Adds `value` to the current counter.
    fn add(&self, value: usize);
    /// Increments by 1 unit the current counter.
    fn inc(&self) {
        self.add(1);
    }
    /// Returns current value of the counter.
    fn count(&self) -> usize;
}

/// Used for defining new types of metrics that do not need a counter and act as a persistent
/// indicator.
pub trait StoreMetric {
    /// Returns current value of the counter.
    fn fetch(&self) -> usize;
    /// Stores `value` to the current counter.
    fn store(&self, value: usize);
}

/// Representation of a metric that is expected to be incremented from more than one thread, so more
/// synchronization is necessary.
// It's currently used for vCPU metrics. An alternative here would be
// to have one instance of every metric for each thread, and to
// aggregate them when writing. However this probably overkill unless we have a lot of vCPUs
// incrementing metrics very often. Still, it's there if we ever need it :-s
// We will be keeping two values for each metric for being able to reset
// counters on each metric.
// 1st member - current value being updated
// 2nd member - old value that gets the current value whenever metrics is flushed to disk
#[derive(Debug, Default)]
pub struct SharedIncMetric(AtomicUsize, AtomicUsize);
impl SharedIncMetric {
    /// Const default construction.
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0), AtomicUsize::new(0))
    }
}

/// Representation of a metric that is expected to hold a value that can be accessed
/// from more than one thread, so more synchronization is necessary.
#[derive(Debug, Default)]
pub struct SharedStoreMetric(AtomicUsize);
impl SharedStoreMetric {
    /// Const default construction.
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }
}

impl IncMetric for SharedIncMetric {
    // While the order specified for this operation is still Relaxed, the actual instruction will
    // be an asm "LOCK; something" and thus atomic across multiple threads, simply because of the
    // fetch_and_add (as opposed to "store(load() + 1)") implementation for atomics.
    // TODO: would a stronger ordering make a difference here?
    fn add(&self, value: usize) {
        self.0.fetch_add(value, Ordering::Relaxed);
    }

    fn count(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}

impl StoreMetric for SharedStoreMetric {
    fn fetch(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }

    fn store(&self, value: usize) {
        self.0.store(value, Ordering::Relaxed);
    }
}

impl Serialize for SharedIncMetric {
    /// Reset counters of each metrics. Here we suppose that Serialize's goal is to help with the
    /// flushing of metrics.
    /// !!! Any print of the metrics will also reset them. Use with caution !!!
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // There's no serializer.serialize_usize() for some reason :(
        let snapshot = self.0.load(Ordering::Relaxed);
        let res = serializer.serialize_u64(snapshot as u64 - self.1.load(Ordering::Relaxed) as u64);

        if res.is_ok() {
            self.1.store(snapshot, Ordering::Relaxed);
        }
        res
    }
}

impl Serialize for SharedStoreMetric {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.load(Ordering::Relaxed) as u64)
    }
}

/// Network-related metrics.
#[derive(Debug, Default, Serialize)]
pub struct NetDeviceMetrics {
    /// Number of times when activate failed on a network device.
    pub activate_fails: SharedIncMetric,
    /// Number of bytes received.
    pub rx_bytes_count: SharedIncMetric,
}
impl NetDeviceMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: SharedIncMetric::new(),
            rx_bytes_count: SharedIncMetric::new(),
        }
    }
}

/// Block Device associated metrics.
#[derive(Debug, Default, Serialize)]
pub struct BlockDeviceMetrics {
    /// Number of times when activate failed on a block device.
    pub activate_fails: SharedIncMetric,
    /// Number of bytes read by this block device.
    pub read_bytes: SharedIncMetric,
}
impl BlockDeviceMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: SharedIncMetric::new(),
            read_bytes: SharedIncMetric::new(),
        }
    }
}

/// Metrics for the seccomp filtering.
#[derive(Debug, Default, Serialize)]
pub struct SeccompMetrics {
    /// Number of errors inside the seccomp filtering.
    pub num_faults: SharedStoreMetric,
}
impl SeccompMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            num_faults: SharedStoreMetric::new(),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
/////////// BTreeMap in SharedIncMetricPerDev
//////////////////////////////////////////////////////////////////////////////////////////
pub trait IncMetricPerDev {
    /// Adds `value` to the current counter.
    fn add(&self, dev: &String, value: usize);
}

#[derive(Default)]
pub struct SharedIncMetricPerDev(Mutex<Cell<BTreeMap<String, (AtomicUsize,AtomicUsize)>>>);

impl SharedIncMetricPerDev {
    /// Const default construction.
    pub fn new() -> Self {
        Self {
            0: Mutex::new(
                Cell::new(BTreeMap::from([
                            (
                                String::from("vsock"),
                                (AtomicUsize::new(0), AtomicUsize::new(0)),
                            ),])
                )
            ),
        }
    }
}

impl Debug for SharedIncMetricPerDev {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedIncMetricPerDev")
            .field("0", &self.0.lock().unwrap().get_mut())
            .finish()
    }
}

impl IncMetricPerDev for SharedIncMetricPerDev {
    // While the order specified for this operation is still Relaxed, the actual instruction will
    // be an asm "LOCK; something" and thus atomic across multiple threads, simply because of the
    // fetch_and_add (as opposed to "store(load() + 1)") implementation for atomics.
    // TODO: would a stronger ordering make a difference here?
    fn add(&self, dev:&String, value: usize) {
        if let Ok(mut mapcell) = self.0.lock() {
            let mapvalue = mapcell.get_mut();
            // println!(">> {:?}", mapvalue);
            if mapvalue.contains_key(dev) {
                println!("{} already exists", dev);
            }  else {
                mapvalue.insert(String::from(dev), (AtomicUsize::new(0),AtomicUsize::new(0)));
                // println!("<<{:?}", value);
            }
            mapvalue[dev].0.fetch_add(value, Ordering::Relaxed);
            mapvalue["vsock"].0.fetch_add(value, Ordering::Relaxed);
        }
    }
}

/// Network-related metrics.
#[derive(Debug, Default, Serialize)]
pub struct VsockMetrics {
    /// Number of times when activate failed on a network device.
    #[serde(skip_serializing)]
    // #[serde(flatten, with = "as_perdev")]
    pub activate_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of bytes received.
    #[serde(skip_serializing)]
    pub rx_bytes_count: Lazy<SharedIncMetricPerDev>,
}

mod as_perdev {
    use serde::ser::{Serializer, SerializeMap};
    use serde::Serialize;
    use crate::metrics::VsockMetrics;
    use std::collections::BTreeMap;
    use std::sync::atomic::Ordering;
    #[derive(Serialize)]
    struct VsockMetricsSerialized {
        pub activate_fails: u64,
        pub rx_bytes_count: u64,
    }
    pub fn serialize<S>(base: &VsockMetrics, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vsockmetric: BTreeMap<String, VsockMetricsSerialized> = BTreeMap::new();
        let mut binding = base.activate_fails.0.lock().unwrap();

        macro_rules! mymacro {
            ($cfield:ident) => {
                let activate_fails_map = binding.get_mut();
                for (k,v) in activate_fails_map.iter() {
                    let snapshot = v.0.load(Ordering::Relaxed);
                    let metr = snapshot as u64 - v.1.load(Ordering::Relaxed) as u64;
                    v.1.store(snapshot, Ordering::Relaxed);
                    if vsockmetric.contains_key(k) {
                        vsockmetric.get_mut(k).unwrap().$cfield = metr;
                    }else{
                        vsockmetric.insert(String::from(k), VsockMetricsSerialized{
                            activate_fails: metr,
                            rx_bytes_count: 0,
                        });
                    }
                }
            };
        }
        mymacro!(activate_fails);
        mymacro!(rx_bytes_count);

        let mut seq = serializer.serialize_map(Some(vsockmetric.len()))?;
        for (k, v) in vsockmetric.iter() {
            seq.serialize_entry(k, &v)?;
        }
        seq.end()
    }
}


impl VsockMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: Lazy::new(SharedIncMetricPerDev::new),
            rx_bytes_count: Lazy::new(SharedIncMetricPerDev::new),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////////
/////////// BTreeMap in PerDevBlockDeviceMetrics
//////////////////////////////////////////////////////////////////////////////////////////

/// Trait for adding metrics to a device.
pub trait PerDevMetrics {
    type MetricType;
    fn new() -> Self;
    fn add(&self, dev: &String, metric: &'static str, value: usize);
    fn get_metrics(&self) -> &Mutex<Cell<BTreeMap<std::string::String, Self::MetricType>>>;
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
     where S: Serializer, Self::MetricType: Serialize {
        use serde::ser::SerializeMap;
        let metrics: &Mutex<Cell<BTreeMap<std::string::String, Self::MetricType>>> = self.get_metrics();
        if let Ok(mut perdevmetric_cell) = metrics.lock() {
            let perdevmetricmap = perdevmetric_cell.get_mut();
            let mut seq = serializer.serialize_map(Some(perdevmetricmap.len()))?;
            for (k, v) in perdevmetricmap.iter() {
                seq.serialize_entry(k, v)?;
            }
            seq.end()
        }  else {
            Err(serde::ser::Error::custom("Failed to lock map"))
        }
    }
}

pub struct PerDevBlockDeviceMetrics {
    pub metrics: Mutex<Cell<BTreeMap<String, BlockDeviceMetrics>>>,
}

// pub struct PerDevNetDeviceMetrics {
//     pub metrics: Mutex<Cell<BTreeMap<String, NetDeviceMetrics>>>,
// }

// impl PerDevMetrics for PerDevNetDeviceMetrics{
//     type MetricType = NetDeviceMetrics;
//     fn get_metrics(&self) -> &Mutex<Cell<BTreeMap<std::string::String, Self::MetricType>>> {
//         &self.metrics
//     }
//     fn new() -> Self {
//         Self {
//             metrics: Mutex::new(
//                 Cell::new(BTreeMap::from([
//                         (
//                             String::from("net"),
//                             NetDeviceMetrics::new(),
//                         ),
//                     ]),
//                 )
//             )
//         }
//     }
//     fn add(&self, dev: &String, metric: &'static str, value: usize) {
//         if let Ok(mut mapcell) = self.metrics.lock() {
//             let mapvalue = mapcell.get_mut();
//             // println!(">> {:?}", mapvalue);
//             if mapvalue.contains_key(dev) {
//                 println!("{} already exists", dev);
//             }  else {
//                 mapvalue.insert(String::from(dev), NetDeviceMetrics::new());
//                 // println!("<<{:?}", value);
//             }
//             match metric {
//                 "activate_fails" => {
//                     mapvalue["net"].activate_fails.add(value);
//                     mapvalue[dev].activate_fails.add(value);
//                 }
//                 "rx_bytes_count" => {
//                     mapvalue["net"].rx_bytes_count.add(value);
//                     mapvalue[dev].rx_bytes_count.add(value);
//                 },
//                 _ => panic!("Unsupported metric"),
//             }
//         }
//     }
// }

impl PerDevMetrics for PerDevBlockDeviceMetrics{
    type MetricType = BlockDeviceMetrics;
    fn get_metrics(&self) -> &Mutex<Cell<BTreeMap<std::string::String, Self::MetricType>>> {
        &self.metrics
    }
    fn new() -> Self {
        Self {
            metrics: Mutex::new(
                Cell::new(BTreeMap::from([
                        (
                            String::from("block"),
                            BlockDeviceMetrics::new(),
                        ),
                    ]),
                )
            )
        }
    }
    fn add(&self, dev: &String, metric: &'static str, value: usize) {
        if let Ok(mut mapcell) = self.metrics.lock() {
            let mapvalue = mapcell.get_mut();
            // println!(">> {:?}", mapvalue);
            if mapvalue.contains_key(dev) {
                println!("{} already exists", dev);
            }  else {
                mapvalue.insert(dev.to_string(), BlockDeviceMetrics::new());
                // println!("<<{:?}", value);
            }
            match metric {
                "activate_fails" => {
                    mapvalue["block"].activate_fails.add(value);
                    mapvalue[dev].activate_fails.add(value);
                }
                "rx_bytes_count" => {
                    mapvalue["block"].read_bytes.add(value);
                    mapvalue[dev].read_bytes.add(value);
                },
                _ => panic!("Unsupported metric"),
            }
        }
    }
}

// impl Debug for PerDevNetDeviceMetrics {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("PerDevNetDeviceMetrics")
//             .field("map", &self.metrics.lock().unwrap().get_mut())
//             .finish()
//     }
// }

impl Debug for PerDevBlockDeviceMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PerDevBlockDeviceMetrics")
            .field("map", &self.metrics.lock().unwrap().get_mut())
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////

/// Structure storing all metrics while enforcing serialization support on them.
#[derive(Serialize)]
pub struct FirecrackerMetrics {
    // #[serde(flatten, with = "generic_as_perdev")]
    #[serde(flatten, with = "PerDevBlockDeviceMetrics")]
    pub block: Lazy<PerDevBlockDeviceMetrics>,
    /// Metrics related to virtio-vsockets.
    pub net: NetDeviceMetrics,
    /// Metrics related to seccomp filtering.
    pub seccomp: SeccompMetrics,
    // #[serde(flatten)]
    #[serde(flatten, with = "as_perdev")]
    pub vsock: VsockMetrics,
}

impl Default for FirecrackerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for FirecrackerMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirecrackerMetrics")
            .field("block", &self.block)
            .field("net", &self.net)
            .field("seccomp", &self.seccomp)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////
impl FirecrackerMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            block: Lazy::new(PerDevBlockDeviceMetrics::new),
            net: NetDeviceMetrics::new(),
            seccomp: SeccompMetrics::new(),
            vsock: VsockMetrics::new(),
        }
    }
}