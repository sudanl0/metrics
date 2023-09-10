use crate::metrics::{SharedIncMetric, IncMetric, PerDeviceMetricsHelper};
use serde::{Serialize, Serializer, ser::SerializeMap};

///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// METRICS ///////////////////////////////////
///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct NetDeviceMetricsBuilder {
    metrics: Vec<NetDeviceMetrics>,
}
impl NetDeviceMetricsBuilder {
    fn new() -> &'static NetDeviceMetrics {
        unsafe{
            NET_DEV_METRICS_PVT.metrics.push(NetDeviceMetrics::new());
            &NET_DEV_METRICS_PVT.metrics[NET_DEV_METRICS_PVT.metrics.len()-1]
        }
    }
}

/// Contains Network-related metrics per device.
static mut NET_DEV_METRICS_PVT: NetDeviceMetricsBuilder = NetDeviceMetricsBuilder {
    metrics: Vec::new(),
};

pub struct NetDeviceMetricsHelper {}
impl PerDeviceMetricsHelper for NetDeviceMetricsHelper {
    fn activate_fails() {
        NetDeviceMetricsBuilder::new().activate_fails.inc();
    }
    fn serialize_metrics<S:Serializer>(serializer: S)
    -> Result<S::Ok, S::Error>{
        unsafe{
            // +1 to accomodate aggregate net metrics
            let mut seq =
            serializer.serialize_map(
                Some(1+NET_DEV_METRICS_PVT.metrics.len()))?;

            let net_aggregated: NetDeviceMetrics = NET_DEV_METRICS_PVT.metrics
            .iter()
            .fold(NetDeviceMetrics::default(),
                 |mut net_agg, net|{ net_agg.aggregate(net); net_agg});

            seq.serialize_entry("net", &net_aggregated)?;
    
            for i in 0..NET_DEV_METRICS_PVT.metrics.len() {
                let devn = format!("net{}", i);
                seq.serialize_entry(&devn, &NET_DEV_METRICS_PVT.metrics[i])?;
            }
            seq.end()
        }
    }
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

    /// Net metrics are SharedIncMetric where the diff of current vs
    /// old is serialized i.e. serialize_u64(current-old).
    /// So to have the aggregate serialized in same way we need to
    /// fetch the diff of current vs old metrics and add it to the
    /// aggregate.
    fn aggregate(&mut self, other: &super::netdevice::NetDeviceMetrics) {
        self.activate_fails.add(other.activate_fails.fetch_diff());
        self.cfg_fails.add(other.cfg_fails.fetch_diff());
        self.mac_address_updates.add(other.mac_address_updates.fetch_diff());
        self.no_rx_avail_buffer.add(other.no_rx_avail_buffer.fetch_diff());
        self.no_tx_avail_buffer.add(other.no_tx_avail_buffer.fetch_diff());
        self.event_fails.add(other.event_fails.fetch_diff());
        self.rx_queue_event_count.add(other.rx_queue_event_count.fetch_diff());
        self.rx_event_rate_limiter_count.add(other.rx_event_rate_limiter_count.fetch_diff());
        self.rx_partial_writes.add(other.rx_partial_writes.fetch_diff());
        self.rx_rate_limiter_throttled.add(other.rx_rate_limiter_throttled.fetch_diff());
        self.rx_tap_event_count.add(other.rx_tap_event_count.fetch_diff());
        self.rx_bytes_count.add(other.rx_bytes_count.fetch_diff());
        self.rx_packets_count.add(other.rx_packets_count.fetch_diff());
        self.rx_fails.add(other.rx_fails.fetch_diff());
        self.rx_count.add(other.rx_count.fetch_diff());
        self.tap_read_fails.add(other.tap_read_fails.fetch_diff());
        self.tap_write_fails.add(other.tap_write_fails.fetch_diff());
        self.tx_bytes_count.add(other.tx_bytes_count.fetch_diff());
        self.tx_malformed_frames.add(other.tx_malformed_frames.fetch_diff());
        self.tx_fails.add(other.tx_fails.fetch_diff());
        self.tx_count.add(other.tx_count.fetch_diff());
        self.tx_packets_count.add(other.tx_packets_count.fetch_diff());
        self.tx_partial_reads.add(other.tx_partial_reads.fetch_diff());
        self.tx_queue_event_count.add(other.tx_queue_event_count.fetch_diff());
        self.tx_rate_limiter_event_count.add(other.tx_rate_limiter_event_count.fetch_diff());
        self.tx_rate_limiter_throttled.add(other.tx_rate_limiter_throttled.fetch_diff());
        self.tx_spoofed_mac_count.add(other.tx_spoofed_mac_count.fetch_diff());
    }
}

#[allow(dead_code)]
pub struct Net{
    #[allow(dead_code)]
    pub(crate) id: String,
    pub(crate) metrics: &'static NetDeviceMetrics,
}

#[allow(dead_code)]
impl Net{
    pub fn new(id: String) -> Net{
        Net{
            id: id,
            metrics: NetDeviceMetricsBuilder::new()
        }
    }
}
