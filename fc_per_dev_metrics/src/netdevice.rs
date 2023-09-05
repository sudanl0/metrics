use crate::metrics::METRICS;
use crate::metrics::{SharedIncMetric, IncMetric};
use serde::{Serialize, ser::SerializeMap};
use std::sync::MutexGuard;
use std::sync::Mutex;
use paste::paste;

struct NetDeviceMetricsBuilder {
    metrics: Mutex<Vec<NetDeviceMetrics>>,
}
static NET_DEV_METRICS_PVT: NetDeviceMetricsBuilder = NetDeviceMetricsBuilder {
    metrics: Mutex::new(Vec::new()),
};

fn get_metrics() -> MutexGuard<'static, Vec<NetDeviceMetrics>>{
    let metrics = NET_DEV_METRICS_PVT.metrics.lock().unwrap();
    metrics
}

impl NetDeviceMetricsBuilder {
    fn new() -> NetMetricsGateway {
        let mut metrics = get_metrics();
        metrics.push(NetDeviceMetrics::new());
        NetMetricsGateway { id: metrics.len() - 1 }
    }
}

pub fn get_serialized_metrics<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
    S: serde::Serializer {
        let dev = "net".to_string();
        let metrics = get_metrics();
        let mut seq = serializer.serialize_map(Some(1+metrics.len()))?;
        seq.serialize_entry("net", &METRICS.net_aggregate)?;
        for i in 0..metrics.len() {
            let devn = dev.clone() + &i.to_string();
            seq.serialize_entry(&devn, &metrics[i])?;
        }
        seq.end()
}

/// Network-related metrics.
#[derive(Debug, Default, Serialize)]
pub struct NetDeviceMetrics {
    /// Number of times when activate failed on a network device.
    pub activate_fails: SharedIncMetric,
    /// Number of times when interacting with the space config of a network device failed.
    pub cfg_fails: SharedIncMetric,
    //// Number of times the mac address was updated through the config space.
    pub mac_address_updates: SharedIncMetric,
    /// No available buffer for the net device rx queue.
    pub no_rx_avail_buffer: SharedIncMetric,
    /// No available buffer for the net device tx queue.
    pub no_tx_avail_buffer: SharedIncMetric,
    /// Number of times when handling events on a network device failed.
    pub event_fails: SharedIncMetric,
    /// Number of events associated with the receiving queue.
    pub rx_queue_event_count: SharedIncMetric,
    /// Number of events associated with the rate limiter installed on the receiving path.
    pub rx_event_rate_limiter_count: SharedIncMetric,
    /// Number of RX partial writes to guest.
    pub rx_partial_writes: SharedIncMetric,
    /// Number of RX rate limiter throttling events.
    pub rx_rate_limiter_throttled: SharedIncMetric,
    /// Number of events received on the associated tap.
    pub rx_tap_event_count: SharedIncMetric,
    /// Number of bytes received.
    pub rx_bytes_count: SharedIncMetric,
    /// Number of packets received.
    pub rx_packets_count: SharedIncMetric,
    /// Number of errors while receiving data.
    pub rx_fails: SharedIncMetric,
    /// Number of successful read operations while receiving data.
    pub rx_count: SharedIncMetric,
    /// Number of times reading from TAP failed.
    pub tap_read_fails: SharedIncMetric,
    /// Number of times writing to TAP failed.
    pub tap_write_fails: SharedIncMetric,
    /// Number of transmitted bytes.
    pub tx_bytes_count: SharedIncMetric,
    /// Number of malformed TX frames.
    pub tx_malformed_frames: SharedIncMetric,
    /// Number of errors while transmitting data.
    pub tx_fails: SharedIncMetric,
    /// Number of successful write operations while transmitting data.
    pub tx_count: SharedIncMetric,
    /// Number of transmitted packets.
    pub tx_packets_count: SharedIncMetric,
    /// Number of TX partial reads from guest.
    pub tx_partial_reads: SharedIncMetric,
    /// Number of events associated with the transmitting queue.
    pub tx_queue_event_count: SharedIncMetric,
    /// Number of events associated with the rate limiter installed on the transmitting path.
    pub tx_rate_limiter_event_count: SharedIncMetric,
    /// Number of RX rate limiter throttling events.
    pub tx_rate_limiter_throttled: SharedIncMetric,
    /// Number of packets with a spoofed mac, sent by the guest.
    pub tx_spoofed_mac_count: SharedIncMetric,
}
impl NetDeviceMetrics {
    /// Const default construction.
    pub const fn new() -> Self {
        Self {
            activate_fails: SharedIncMetric::new(),
            cfg_fails: SharedIncMetric::new(),
            mac_address_updates: SharedIncMetric::new(),
            no_rx_avail_buffer: SharedIncMetric::new(),
            no_tx_avail_buffer: SharedIncMetric::new(),
            event_fails: SharedIncMetric::new(),
            rx_queue_event_count: SharedIncMetric::new(),
            rx_event_rate_limiter_count: SharedIncMetric::new(),
            rx_partial_writes: SharedIncMetric::new(),
            rx_rate_limiter_throttled: SharedIncMetric::new(),
            rx_tap_event_count: SharedIncMetric::new(),
            rx_bytes_count: SharedIncMetric::new(),
            rx_packets_count: SharedIncMetric::new(),
            rx_fails: SharedIncMetric::new(),
            rx_count: SharedIncMetric::new(),
            tap_read_fails: SharedIncMetric::new(),
            tap_write_fails: SharedIncMetric::new(),
            tx_bytes_count: SharedIncMetric::new(),
            tx_malformed_frames: SharedIncMetric::new(),
            tx_fails: SharedIncMetric::new(),
            tx_count: SharedIncMetric::new(),
            tx_packets_count: SharedIncMetric::new(),
            tx_partial_reads: SharedIncMetric::new(),
            tx_queue_event_count: SharedIncMetric::new(),
            tx_rate_limiter_event_count: SharedIncMetric::new(),
            tx_rate_limiter_throttled: SharedIncMetric::new(),
            tx_spoofed_mac_count: SharedIncMetric::new(),
        }
    }
}

pub trait NetDeviceMetricsFns {
    fn activate_fails_add(&self, n: usize);
    fn cfg_fails_add(&self, n: usize);
    fn mac_address_updates_add(&self, n: usize);
    fn no_rx_avail_buffer_add(&self, n: usize);
    fn no_tx_avail_buffer_add(&self, n: usize);
    fn event_fails_add(&self, n: usize);
    fn rx_queue_event_count_add(&self, n: usize);
    fn rx_event_rate_limiter_count_add(&self, n: usize);
    fn rx_partial_writes_add(&self, n: usize);
    fn rx_rate_limiter_throttled_add(&self, n: usize);
    fn rx_tap_event_count_add(&self, n: usize);
    fn rx_bytes_count_add(&self, n: usize);
    fn rx_packets_count_add(&self, n: usize);
    fn rx_fails_add(&self, n: usize);
    fn rx_count_add(&self, n: usize);
    fn tap_read_fails_add(&self, n: usize);
    fn tap_write_fails_add(&self, n: usize);
    fn tx_bytes_count_add(&self, n: usize);
    fn tx_malformed_frames_add(&self, n: usize);
    fn tx_fails_add(&self, n: usize);
    fn tx_count_add(&self, n: usize);
    fn tx_packets_count_add(&self, n: usize);
    fn tx_partial_reads_add(&self, n: usize);
    fn tx_queue_event_count_add(&self, n: usize);
    fn tx_rate_limiter_event_count_add(&self, n: usize);
    fn tx_rate_limiter_throttled_add(&self, n: usize);
    fn tx_spoofed_mac_count_add(&self, n: usize);
}

macro_rules! metrics_add {
    ($name:ident) => {
        paste! {
            // Defines a const called `QRST`.
            fn [<$name _add>](&self, n: usize) {
                let metrics = get_metrics();
                metrics[self.id].$name.add(n);
                METRICS.net_aggregate.$name.add(n);
            }
        }
    }
}

#[derive(Debug)]
pub struct NetMetricsGateway{
    pub(crate) id: usize,
}

impl NetDeviceMetricsFns for NetMetricsGateway {
    metrics_add!(activate_fails);
    metrics_add!(cfg_fails);
    metrics_add!(mac_address_updates);
    metrics_add!(no_rx_avail_buffer);
    metrics_add!(no_tx_avail_buffer);
    metrics_add!(event_fails);
    metrics_add!(rx_queue_event_count);
    metrics_add!(rx_event_rate_limiter_count);
    metrics_add!(rx_partial_writes);
    metrics_add!(rx_rate_limiter_throttled);
    metrics_add!(rx_tap_event_count);
    metrics_add!(rx_bytes_count);
    metrics_add!(rx_packets_count);
    metrics_add!(rx_fails);
    metrics_add!(rx_count);
    metrics_add!(tap_read_fails);
    metrics_add!(tap_write_fails);
    metrics_add!(tx_bytes_count);
    metrics_add!(tx_malformed_frames);
    metrics_add!(tx_fails);
    metrics_add!(tx_count);
    metrics_add!(tx_packets_count);
    metrics_add!(tx_partial_reads);
    metrics_add!(tx_queue_event_count);
    metrics_add!(tx_rate_limiter_event_count);
    metrics_add!(tx_rate_limiter_throttled);
    metrics_add!(tx_spoofed_mac_count);
}

pub struct Net{
    #[allow(dead_code)]
    pub(crate) id: String,
    pub(crate) metrics: NetMetricsGateway,
}

impl Net{
    pub fn new(id: String) -> Net{
        Net{
            id: id,
            metrics: NetDeviceMetricsBuilder::new()
        }
    }
}