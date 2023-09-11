mod metrics;
mod netdevice;
use crate::metrics::{METRICS, METRICSDUMMY, Metrics, FirecrackerMetrics};
use std::time::SystemTime;
use std::io::LineWriter;
use std::fs::File;
use crate::netdevice::Net;
use crate::metrics::IncMetric;

fn test_net_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
// /*
    let net0 = Net::new(String::from("net0"));
    let net1 = Net::new(String::from("net1"));
    let t0 = SystemTime::now();
    net0.metrics.cfg_fails.add(10);
    net0.metrics.mac_address_updates.add(10);
    net0.metrics.no_rx_avail_buffer.inc();
    net0.metrics.no_tx_avail_buffer.inc();
    net0.metrics.event_fails.inc();
    net0.metrics.rx_queue_event_count.inc();
    net0.metrics.rx_event_rate_limiter_count.inc();
    net0.metrics.rx_partial_writes.add(10);
    net0.metrics.rx_rate_limiter_throttled.add(10);
    net0.metrics.rx_tap_event_count.add(10);
    net0.metrics.rx_bytes_count.add(10);
    net0.metrics.rx_packets_count.add(10);
    net0.metrics.rx_fails.add(10);
    net0.metrics.rx_count.add(10);
    net0.metrics.tap_read_fails.add(10);
    net0.metrics.tap_write_fails.add(10);
    net0.metrics.tx_bytes_count.add(10);
    net0.metrics.tx_malformed_frames.add(10);
    net0.metrics.tx_fails.add(10);
    net0.metrics.tx_count.add(10);
    net0.metrics.tx_packets_count.add(10);
    net0.metrics.tx_partial_reads.add(10);
    net0.metrics.tx_queue_event_count.add(10);
    net0.metrics.tx_rate_limiter_event_count.add(10);
    net0.metrics.tx_rate_limiter_throttled.add(10);
    net0.metrics.tx_spoofed_mac_count.add(10);
    net1.metrics.cfg_fails.add(10);
    net1.metrics.mac_address_updates.add(10);
    net1.metrics.no_rx_avail_buffer.add(10);
    net1.metrics.no_tx_avail_buffer.add(10);
    net1.metrics.event_fails.add(10);
    net1.metrics.rx_queue_event_count.add(10);
    net1.metrics.rx_event_rate_limiter_count.add(10);
    net1.metrics.rx_partial_writes.add(10);
    net1.metrics.rx_rate_limiter_throttled.add(10);
    net1.metrics.rx_tap_event_count.add(10);
    net1.metrics.rx_bytes_count.add(10);
    net1.metrics.rx_packets_count.add(10);
    net1.metrics.rx_fails.add(10);
    net1.metrics.rx_count.add(10);
    net1.metrics.tap_read_fails.add(10);
    net1.metrics.tap_write_fails.add(10);
    net1.metrics.tx_bytes_count.add(10);
    net1.metrics.tx_malformed_frames.add(10);
    net1.metrics.tx_fails.add(10);
    net1.metrics.tx_count.add(10);
    net1.metrics.tx_packets_count.add(10);
    net1.metrics.tx_partial_reads.add(10);
    net1.metrics.tx_queue_event_count.add(10);
    net1.metrics.tx_rate_limiter_event_count.add(10);
    net1.metrics.tx_rate_limiter_throttled.add(10);
    net1.metrics.tx_spoofed_mac_count.add(10);
    let t1 = SystemTime::now();
    println!("Time take to update metrics with proposal: {:?}", t1.duration_since(t0).unwrap());
// */

    let t0 = SystemTime::now();
    assert!(m.write().is_ok());
    let t1 = SystemTime::now();
    println!("Time take to flush metrics with proposal: {:?}", t1.duration_since(t0).unwrap());
}

fn main(){
    let m = &METRICS;
    let md = &METRICSDUMMY;

    let fd = File::create("./metricsd.json").expect("Failed to create temporary metrics file");
    assert!(md.init(LineWriter::new(fd)).is_ok());

    let t0 = SystemTime::now();
    assert!(md.write().is_ok());
    let t1 = SystemTime::now();
    println!("Time take to flush metrics with 3 fields of NetDeviceMetrics: {:?}", t1.duration_since(t0).unwrap());

    let f = File::create("./metrics.json").expect("Failed to create temporary metrics file");
    assert!(m.init(LineWriter::new(f)).is_ok());

    test_net_metrics(m);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_net_metrics_proposal() {
        let m = &METRICS;

        let f = File::create("./metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());

        test_net_metrics(m);
    }
}