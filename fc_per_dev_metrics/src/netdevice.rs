use crate::metrics::{METRICS, SharedIncMetric,IncMetric};
use serde::{Serialize, ser::SerializeMap};

/// Network-related metrics.
#[derive(Debug, Default, Serialize)]
pub struct NetPerDeviceMetrics {
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
impl NetPerDeviceMetrics {
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
    fn rx_bytes_count_add(&self, n: usize);
}

impl NetDeviceMetricsFns for NetPerDeviceMetrics {
    fn activate_fails_add(&self, n: usize) {
        self.activate_fails.add(n);
        METRICS.net.activate_fails.add(n);
    }
    fn rx_bytes_count_add(&self, n: usize) {
        self.rx_bytes_count.add(n);
        METRICS.net.rx_bytes_count.add(n);
    }
}

// #[derive(Debug, Serialize)]
// #[serde(deny_unknown_fields)]
pub struct Net{
    pub(crate) id: String,
    pub(crate) metrics: NetPerDeviceMetrics,
}

impl Serialize for Net{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let dev = self.id.clone();
        let mut seq = serializer.serialize_map(Some(1))?;
        // println!(">>{:?}", dev);
        seq.serialize_entry(&dev, &self.metrics)?;
        seq.end()
    }
}

impl Net{
    pub fn new(id: String) -> Net{
        Net{
            id,
            metrics: NetPerDeviceMetrics::new()
        }
    }
}