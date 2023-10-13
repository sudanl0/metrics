// use std::fmt::{Debug, format};
use std::fmt::Debug;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::vec;
use std::collections::BTreeMap;

use serde::{Serialize, Serializer, Deserialize, ser::SerializeMap};
use crate::netdevice::flush_metrics;

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
#[derive(Debug, Deserialize)]
pub struct EMFMetrics{
    inner: BTreeMap<String,usize>,
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

    pub fn print_emf(&self, fcmetrics: String) {
        #[derive(Debug, Serialize,Deserialize)]
        struct Emf{
            utc_timestamp_ms: usize,
            #[serde(flatten)]
            innerm: BTreeMap<String,BTreeMap<String,usize>>,
        }
        #[derive(Debug, Serialize,Deserialize)]
        struct Metric{
            #[serde(rename = "Name")]
            name: String,
            #[serde(rename = "Unit")]
            unit: String,
        }
        #[derive(Debug, Serialize,Deserialize)]
        struct MetricDirective{
            #[serde(rename = "Namespace")]
            namespace: String,
            #[serde(rename = "Dimensions")]
            dimensions: Vec<Vec<String>>,
            #[serde(rename = "Metrics")]
            metrics: Vec<Metric>,
            // Metrics: Vec<BTreeMap<String,String>>,
        }
        #[derive(Debug, Serialize,Deserialize)]
        struct MetricDirectiveObj{
            #[serde(rename = "Timestamp")]
            timestamp: usize,
            #[serde(rename = "CloudWatchMetrics")]
            cloud_watch_metrics: Vec<MetricDirective>,
        }

        mod as_emf_metrics{
            use super::*;
            pub fn serialize<S>(metrics: &Vec<EMFMetrics>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                {
                    let mut seq = serializer.serialize_map(Some(metrics.len()))?;
                    for metric in metrics.iter() {
                        for (key,value) in (*metric).inner.iter(){
                            seq.serialize_entry(key,value)?;
                        }
                    }
                    seq.end()
                }
        }
        #[derive(Debug, Serialize)]
        struct EmfStruct {
            #[serde(flatten)]
            aws: BTreeMap<String,MetricDirectiveObj>,
            #[serde(rename = "SandboxId")]
            sandbox_id: usize,
            #[serde(flatten, with = "as_emf_metrics")]
            metrics: Vec<EMFMetrics>,
        }
        let mut final_emf = EmfStruct{
            aws: BTreeMap::new(),
            sandbox_id: 1234,
            metrics: Vec::new(),
        };
        final_emf.aws.insert("_aws".to_string(),
                MetricDirectiveObj{
                timestamp: 0,
                cloud_watch_metrics: vec![MetricDirective{
                namespace: "TestNs".to_string(),
                dimensions: Vec::new(),
                metrics: Vec::new(),
                }]
            }
        );
        fn get_unit(key: &str) -> String{
            let mut unit = "Count".to_string();
            if key.to_lowercase().ends_with("_bytes") || key.to_lowercase().ends_with("_bytes_count"){
                unit = "Bytes".to_string();
            }else if key.to_lowercase().ends_with("_ms"){
                unit = "Milliseconds".to_string();
            }else if key.to_lowercase().ends_with("_us") {
                unit = "Microseconds".to_string()
            }
            unit
        }
        let emf = serde_json::from_str::<Emf>(fcmetrics.as_str()).unwrap();
        // Timestamp = emf.utc_timestamp_ms;
        let mobj = final_emf.aws.get_mut("_aws").unwrap();
        mobj.timestamp = emf.utc_timestamp_ms;
        mobj.cloud_watch_metrics[0].dimensions.push(vec!["Sandbox".to_string()]);
        for (key,value) in emf.innerm.iter(){
            for (k,v) in value.iter(){
                let emfmetrics = EMFMetrics{
                    inner: BTreeMap::from([(format!("{}.{}", key, k),*v)])
                };
                final_emf.metrics.push(emfmetrics);
                mobj.cloud_watch_metrics[0].metrics.push(
                    Metric{
                        name: format!("{}.{}", key, k),
                        unit: get_unit(k),
                    }
                );
            }
        }
        println!("{}",serde_json::to_string_pretty(&final_emf).unwrap());
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
                    // self.print_emf(msg.clone());
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
    /// Returns diff of current and old value of the counter.
    fn fetch_diff(&self) -> usize;
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

    fn fetch_diff(&self) -> usize {
        let res = self.0.load(Ordering::Relaxed) - self.1.load(Ordering::Relaxed);
        res
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

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
/// Trait to be implemented by all devices having metrics that need to be tracked.
pub trait PerDeviceMetricsHelper{
    /// each device implements this function to serialize its metrics
    fn serialize_metrics<S:Serializer>(serializer: S) -> Result<S::Ok, S::Error>;
}

#[derive(Default, Debug)]
pub struct NetDeviceMetricsDummmy{}
impl NetDeviceMetricsDummmy{
    pub const fn new() -> Self{
        Self{}
    }
}

impl Serialize for NetDeviceMetricsDummmy{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
                flush_metrics(serializer)
    }
}

#[derive(Debug, Default)]
struct SerializeToUtcTimestampMs;
impl SerializeToUtcTimestampMs {
    /// Const default construction.
    pub const fn new() -> Self {
        SerializeToUtcTimestampMs
    }
}

#[derive(Debug)]
pub enum ClockType {
    /// Equivalent to `libc::CLOCK_MONOTONIC`.
    Monotonic,
    /// Equivalent to `libc::CLOCK_REALTIME`.
    Real,
    /// Equivalent to `libc::CLOCK_PROCESS_CPUTIME_ID`.
    ProcessCpu,
    /// Equivalent to `libc::CLOCK_THREAD_CPUTIME_ID`.
    ThreadCpu,
}
pub const NANOS_PER_SECOND: u64 = 1_000_000_000;
pub fn seconds_to_nanoseconds(value: i64) -> Option<i64> {
    value.checked_mul(i64::try_from(NANOS_PER_SECOND).unwrap())
}
impl From<ClockType> for libc::clockid_t {
    fn from(clock_type: ClockType) -> Self {
        match clock_type {
            ClockType::Monotonic => libc::CLOCK_MONOTONIC,
            ClockType::Real => libc::CLOCK_REALTIME,
            ClockType::ProcessCpu => libc::CLOCK_PROCESS_CPUTIME_ID,
            ClockType::ThreadCpu => libc::CLOCK_THREAD_CPUTIME_ID,
        }
    }
}
pub fn get_time_ns(clock_type: ClockType) -> u64 {
    let mut time_struct = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: Safe because the parameters are valid.
    unsafe { libc::clock_gettime(clock_type.into(), &mut time_struct) };
    u64::try_from(seconds_to_nanoseconds(time_struct.tv_sec).expect("Time conversion overflow"))
        .unwrap()
        + u64::try_from(time_struct.tv_nsec).unwrap()
}

impl Serialize for SerializeToUtcTimestampMs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(
            i64::try_from(get_time_ns(ClockType::Real) / 1_000_000)
                .unwrap(),
        )
    }
}
/// Structure storing all metrics while enforcing serialization support on them.
#[derive(Serialize)]
pub struct FirecrackerMetrics {
    utc_timestamp_ms: SerializeToUtcTimestampMs,
    #[serde(flatten)]
    pub net: NetDeviceMetricsDummmy,
}

impl Default for FirecrackerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for FirecrackerMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirecrackerMetrics")
            .field("net", &self.net)
            .finish()
    }
}

///////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////
impl FirecrackerMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            utc_timestamp_ms: SerializeToUtcTimestampMs::new(),
            net: NetDeviceMetricsDummmy::new(),
        }
    }
}
