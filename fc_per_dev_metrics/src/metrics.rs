
use std::fmt::Debug;
use std::io::Write;
use std::ops::Deref;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use once_cell::sync::Lazy;
use std::collections::BTreeMap;
use std::cell::Cell;
use crate::netdevice::{get_serialized_metrics, NetDeviceMetrics};

use serde::{Serialize, Serializer};

pub type FcLineWriter = std::io::LineWriter<std::fs::File>;

/// Static instance used for handling metrics.
pub static METRICS: Metrics<FirecrackerMetrics, FcLineWriter> =
    Metrics::<FirecrackerMetrics, FcLineWriter>::new(FirecrackerMetrics::new());
#[allow(unused)]
pub static METRICS1: Metrics<FirecrackerMetrics, FcLineWriter> =
    Metrics::<FirecrackerMetrics, FcLineWriter>::new(FirecrackerMetrics::new());
#[allow(unused)]
pub static METRICS3: Metrics<FirecrackerMetrics, FcLineWriter> =
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
#[derive(Debug, Default)]
pub struct VsockMetrics {
    // #[serde(flatten, with = "as_perdev")]
    /// Number of times when activate failed on a network device.
    pub activate_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of times when interacting with the space config of a network device failed.
    pub cfg_fails: Lazy<SharedIncMetricPerDev>,
    //// Number of times the mac address was updated through the config space.
    pub mac_address_updates: Lazy<SharedIncMetricPerDev>,
    /// No available buffer for the net device rx queue.
    pub no_rx_avail_buffer: Lazy<SharedIncMetricPerDev>,
    /// No available buffer for the net device tx queue.
    pub no_tx_avail_buffer: Lazy<SharedIncMetricPerDev>,
    /// Number of times when handling events on a network device failed.
    pub event_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of events associated with the receiving queue.
    pub rx_queue_event_count: Lazy<SharedIncMetricPerDev>,
    /// Number of events associated with the rate limiter installed on the receiving path.
    pub rx_event_rate_limiter_count: Lazy<SharedIncMetricPerDev>,
    /// Number of RX partial writes to guest.
    pub rx_partial_writes: Lazy<SharedIncMetricPerDev>,
    /// Number of RX rate limiter throttling events.
    pub rx_rate_limiter_throttled: Lazy<SharedIncMetricPerDev>,
    /// Number of events received on the associated tap.
    pub rx_tap_event_count: Lazy<SharedIncMetricPerDev>,
    /// Number of bytes received.
    pub rx_bytes_count: Lazy<SharedIncMetricPerDev>,
    /// Number of packets received.
    pub rx_packets_count: Lazy<SharedIncMetricPerDev>,
    /// Number of errors while receiving data.
    pub rx_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of successful read operations while receiving data.
    pub rx_count: Lazy<SharedIncMetricPerDev>,
    /// Number of times reading from TAP failed.
    pub tap_read_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of times writing to TAP failed.
    pub tap_write_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of transmitted bytes.
    pub tx_bytes_count: Lazy<SharedIncMetricPerDev>,
    /// Number of malformed TX frames.
    pub tx_malformed_frames: Lazy<SharedIncMetricPerDev>,
    /// Number of errors while transmitting data.
    pub tx_fails: Lazy<SharedIncMetricPerDev>,
    /// Number of successful write operations while transmitting data.
    pub tx_count: Lazy<SharedIncMetricPerDev>,
    /// Number of transmitted packets.
    pub tx_packets_count: Lazy<SharedIncMetricPerDev>,
    /// Number of TX partial reads from guest.
    pub tx_partial_reads: Lazy<SharedIncMetricPerDev>,
    /// Number of events associated with the transmitting queue.
    pub tx_queue_event_count: Lazy<SharedIncMetricPerDev>,
    /// Number of events associated with the rate limiter installed on the transmitting path.
    pub tx_rate_limiter_event_count: Lazy<SharedIncMetricPerDev>,
    /// Number of RX rate limiter throttling events.
    pub tx_rate_limiter_throttled: Lazy<SharedIncMetricPerDev>,
    /// Number of packets with a spoofed mac, sent by the guest.
    pub tx_spoofed_mac_count: Lazy<SharedIncMetricPerDev>,
}

impl VsockMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: Lazy::new(SharedIncMetricPerDev::new),
            cfg_fails: Lazy::new(SharedIncMetricPerDev::new),
            mac_address_updates: Lazy::new(SharedIncMetricPerDev::new),
            no_rx_avail_buffer: Lazy::new(SharedIncMetricPerDev::new),
            no_tx_avail_buffer: Lazy::new(SharedIncMetricPerDev::new),
            event_fails: Lazy::new(SharedIncMetricPerDev::new),
            rx_queue_event_count: Lazy::new(SharedIncMetricPerDev::new),
            rx_event_rate_limiter_count: Lazy::new(SharedIncMetricPerDev::new),
            rx_partial_writes: Lazy::new(SharedIncMetricPerDev::new),
            rx_rate_limiter_throttled: Lazy::new(SharedIncMetricPerDev::new),
            rx_tap_event_count: Lazy::new(SharedIncMetricPerDev::new),
            rx_bytes_count: Lazy::new(SharedIncMetricPerDev::new),
            rx_packets_count: Lazy::new(SharedIncMetricPerDev::new),
            rx_fails: Lazy::new(SharedIncMetricPerDev::new),
            rx_count: Lazy::new(SharedIncMetricPerDev::new),
            tap_read_fails: Lazy::new(SharedIncMetricPerDev::new),
            tap_write_fails: Lazy::new(SharedIncMetricPerDev::new),
            tx_bytes_count: Lazy::new(SharedIncMetricPerDev::new),
            tx_malformed_frames: Lazy::new(SharedIncMetricPerDev::new),
            tx_fails: Lazy::new(SharedIncMetricPerDev::new),
            tx_count: Lazy::new(SharedIncMetricPerDev::new),
            tx_packets_count: Lazy::new(SharedIncMetricPerDev::new),
            tx_partial_reads: Lazy::new(SharedIncMetricPerDev::new),
            tx_queue_event_count: Lazy::new(SharedIncMetricPerDev::new),
            tx_rate_limiter_event_count: Lazy::new(SharedIncMetricPerDev::new),
            tx_rate_limiter_throttled: Lazy::new(SharedIncMetricPerDev::new),
            tx_spoofed_mac_count: Lazy::new(SharedIncMetricPerDev::new),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////

mod as_net {
    // use serde::ser::SerializeMap;
    use super::*;

    pub fn serialize<S>(_base: &NetDeviceMetrics, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
                get_serialized_metrics(serializer)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////////

/// Structure storing all metrics while enforcing serialization support on them.
#[derive(Serialize)]
pub struct FirecrackerMetrics {
    /// Metrics related to seccomp filtering.
    pub seccomp: SeccompMetrics,
    #[serde(skip)]
    pub vsock: VsockMetrics,
    #[serde(flatten, with = "as_net")]
    pub net_aggregate: NetDeviceMetrics
}

impl Default for FirecrackerMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for FirecrackerMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirecrackerMetrics")
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
            seccomp: SeccompMetrics::new(),
            vsock: VsockMetrics::new(),
            net_aggregate: NetDeviceMetrics::new(),
        }
    }
}
