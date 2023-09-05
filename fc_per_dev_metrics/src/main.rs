mod metrics;
mod netdevice;
use crate::metrics::{METRICS, Metrics, FirecrackerMetrics};
use crate::metrics::IncMetric;
use crate::metrics::IncMetricPerDev;
use crate::netdevice::Net;
use crate::netdevice::NetDeviceMetricsFns;
use std::time::SystemTime;
use std::io::LineWriter;
use std::fs::File;

fn test_vsock_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
    let t0 = SystemTime::now();
    METRICS.vsock.activate_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.cfg_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.mac_address_updates.add(&String::from("vsock0"), 10);
    METRICS.vsock.no_rx_avail_buffer.add(&String::from("vsock0"), 10);
    METRICS.vsock.no_tx_avail_buffer.add(&String::from("vsock0"), 10);
    METRICS.vsock.event_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_queue_event_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_event_rate_limiter_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_partial_writes.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_rate_limiter_throttled.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_tap_event_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_bytes_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_packets_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.rx_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tap_read_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.tap_write_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_bytes_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_malformed_frames.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_packets_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_partial_reads.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_queue_event_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_rate_limiter_event_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_rate_limiter_throttled.add(&String::from("vsock0"), 10);
    METRICS.vsock.tx_spoofed_mac_count.add(&String::from("vsock0"), 10);
    METRICS.vsock.activate_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.cfg_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.mac_address_updates.add(&String::from("vsock1"), 10);
    METRICS.vsock.no_rx_avail_buffer.add(&String::from("vsock1"), 10);
    METRICS.vsock.no_tx_avail_buffer.add(&String::from("vsock1"), 10);
    METRICS.vsock.event_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_queue_event_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_event_rate_limiter_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_partial_writes.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_rate_limiter_throttled.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_tap_event_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_bytes_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_packets_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.rx_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tap_read_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.tap_write_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_bytes_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_malformed_frames.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_fails.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_packets_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_partial_reads.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_queue_event_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_rate_limiter_event_count.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_rate_limiter_throttled.add(&String::from("vsock1"), 10);
    METRICS.vsock.tx_spoofed_mac_count.add(&String::from("vsock1"), 10);
    let t1 = SystemTime::now();
    println!("Time take to update metrics when BTreeMap is part of SharedIncMetrics: {:?}", t1.duration_since(t0).unwrap());
    let t0 = SystemTime::now();
    assert!(m.write().is_ok());
    let t1 = SystemTime::now();
    println!("Time take to flush metrics when BTreeMap is part of SharedIncMetrics: {:?}", t1.duration_since(t0).unwrap());
}

fn test_net_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
    let t0 = SystemTime::now();

    METRICS.net_aggregate.activate_fails.add(10);
    METRICS.net_aggregate.cfg_fails.add(10);
    METRICS.net_aggregate.mac_address_updates.add(10);
    METRICS.net_aggregate.no_rx_avail_buffer.add(10);
    METRICS.net_aggregate.no_tx_avail_buffer.add(10);
    METRICS.net_aggregate.event_fails.add(10);
    METRICS.net_aggregate.rx_queue_event_count.add(10);
    METRICS.net_aggregate.rx_event_rate_limiter_count.add(10);
    METRICS.net_aggregate.rx_partial_writes.add(10);
    METRICS.net_aggregate.rx_rate_limiter_throttled.add(10);
    METRICS.net_aggregate.rx_tap_event_count.add(10);
    METRICS.net_aggregate.rx_bytes_count.add(10);
    METRICS.net_aggregate.rx_packets_count.add(10);
    METRICS.net_aggregate.rx_fails.add(10);
    METRICS.net_aggregate.rx_count.add(10);
    METRICS.net_aggregate.tap_read_fails.add(10);
    METRICS.net_aggregate.tap_write_fails.add(10);
    METRICS.net_aggregate.tx_bytes_count.add(10);
    METRICS.net_aggregate.tx_malformed_frames.add(10);
    METRICS.net_aggregate.tx_fails.add(10);
    METRICS.net_aggregate.tx_count.add(10);
    METRICS.net_aggregate.tx_packets_count.add(10);
    METRICS.net_aggregate.tx_partial_reads.add(10);
    METRICS.net_aggregate.tx_queue_event_count.add(10);
    METRICS.net_aggregate.tx_rate_limiter_event_count.add(10);
    METRICS.net_aggregate.tx_rate_limiter_throttled.add(10);
    METRICS.net_aggregate.tx_spoofed_mac_count.add(10);
// /*
    let net0 = Net::new(String::from("net0"));
    let net1 = Net::new(String::from("net1"));
    net0.metrics.activate_fails_add(10);
    net0.metrics.cfg_fails_add(10);
    net0.metrics.mac_address_updates_add(10);
    net0.metrics.no_rx_avail_buffer_add(10);
    net0.metrics.no_tx_avail_buffer_add(10);
    net0.metrics.event_fails_add(10);
    net0.metrics.rx_queue_event_count_add(10);
    net0.metrics.rx_event_rate_limiter_count_add(10);
    net0.metrics.rx_partial_writes_add(10);
    net0.metrics.rx_rate_limiter_throttled_add(10);
    net0.metrics.rx_tap_event_count_add(10);
    net0.metrics.rx_bytes_count_add(10);
    net0.metrics.rx_packets_count_add(10);
    net0.metrics.rx_fails_add(10);
    net0.metrics.rx_count_add(10);
    net0.metrics.tap_read_fails_add(10);
    net0.metrics.tap_write_fails_add(10);
    net0.metrics.tx_bytes_count_add(10);
    net0.metrics.tx_malformed_frames_add(10);
    net0.metrics.tx_fails_add(10);
    net0.metrics.tx_count_add(10);
    net0.metrics.tx_packets_count_add(10);
    net0.metrics.tx_partial_reads_add(10);
    net0.metrics.tx_queue_event_count_add(10);
    net0.metrics.tx_rate_limiter_event_count_add(10);
    net0.metrics.tx_rate_limiter_throttled_add(10);
    net0.metrics.tx_spoofed_mac_count_add(10);
    net1.metrics.activate_fails_add(10);
    net1.metrics.cfg_fails_add(10);
    net1.metrics.mac_address_updates_add(10);
    net1.metrics.no_rx_avail_buffer_add(10);
    net1.metrics.no_tx_avail_buffer_add(10);
    net1.metrics.event_fails_add(10);
    net1.metrics.rx_queue_event_count_add(10);
    net1.metrics.rx_event_rate_limiter_count_add(10);
    net1.metrics.rx_partial_writes_add(10);
    net1.metrics.rx_rate_limiter_throttled_add(10);
    net1.metrics.rx_tap_event_count_add(10);
    net1.metrics.rx_bytes_count_add(10);
    net1.metrics.rx_packets_count_add(10);
    net1.metrics.rx_fails_add(10);
    net1.metrics.rx_count_add(10);
    net1.metrics.tap_read_fails_add(10);
    net1.metrics.tap_write_fails_add(10);
    net1.metrics.tx_bytes_count_add(10);
    net1.metrics.tx_malformed_frames_add(10);
    net1.metrics.tx_fails_add(10);
    net1.metrics.tx_count_add(10);
    net1.metrics.tx_packets_count_add(10);
    net1.metrics.tx_partial_reads_add(10);
    net1.metrics.tx_queue_event_count_add(10);
    net1.metrics.tx_rate_limiter_event_count_add(10);
    net1.metrics.tx_rate_limiter_throttled_add(10);
    net1.metrics.tx_spoofed_mac_count_add(10);
// */
    let t1 = SystemTime::now();
    println!("Time take to update metrics when they are part of Net: {:?}", t1.duration_since(t0).unwrap());
    assert!(m.write().is_ok());
    let t0 = SystemTime::now();
    let t1 = SystemTime::now();
    println!("Time take to flush metrics when they are part of Net: {:?}", t1.duration_since(t0).unwrap());
}

fn main(){
    let m = &METRICS;

    let res = m.write();
    assert!(res.is_ok() && !res.unwrap());

    let f = File::create("./metrics.json").expect("Failed to create temporary metrics file");
    assert!(m.init(LineWriter::new(f)).is_ok());
    assert!(m.write().is_ok());

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// BTreeMap in SharedIncMetricPerDev
    //////////////////////////////////////////////////////////////////////////////////////////

    test_vsock_metrics(m);
    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// NetDeviceMetrics moved to Net as NetPerDeviceMetrics with
    /////////// aggregate using trait and no BTreeMap
    //////////////////////////////////////////////////////////////////////////////////////////

    test_net_metrics(m);
}

#[cfg(test)]
mod tests {
    use crate::metrics::{METRICS1, METRICS3};
    use crate::{test_vsock_metrics, test_net_metrics};
    use std::io::LineWriter;
    use std::fs::File;

    #[test]
    fn test_vsock_metrics_proposal() {
        let m = &METRICS1;
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());
        
        let f = File::create("./vsock_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());

        test_vsock_metrics(m);
    }

    #[test]
    fn test_net_metrics_proposal() {
        use std::io::LineWriter;
        use std::fs::File;
        let m = &METRICS3;
        
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());

        let f = File::create("./net_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());

        test_net_metrics(m);
    }
}