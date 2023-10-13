mod metrics;
mod netdevice;
use crate::metrics::{METRICS, Metrics, FirecrackerMetrics};
use std::time::SystemTime;
use std::io::LineWriter;
use std::fs::File;
use crate::netdevice::Net;
use crate::metrics::IncMetric;

fn test_net_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
// /*
    let net0 = Net::new(String::from("eth00"));
    let net1 = Net::new(String::from("eth11"));
    let t0 = SystemTime::now();
    net0.metrics.get().write().unwrap().activate_fails.inc();
    net0.metrics.get().write().unwrap().cfg_fails.add(10);
    net0.metrics.get().write().unwrap().mac_address_updates.add(10);
    net0.metrics.get().write().unwrap().no_rx_avail_buffer.inc();
    net0.metrics.get().write().unwrap().no_tx_avail_buffer.inc();
    net0.metrics.get().write().unwrap().event_fails.inc();
    net0.metrics.get().write().unwrap().rx_queue_event_count.inc();
    net0.metrics.get().write().unwrap().rx_event_rate_limiter_count.inc();
    net0.metrics.get().write().unwrap().rx_partial_writes.add(10);
    net0.metrics.get().write().unwrap().rx_rate_limiter_throttled.add(10);
    net0.metrics.get().write().unwrap().rx_tap_event_count.add(10);
    net0.metrics.get().write().unwrap().rx_bytes_count.add(10);
    net0.metrics.get().write().unwrap().rx_packets_count.add(10);
    net0.metrics.get().write().unwrap().rx_fails.add(10);
    net0.metrics.get().write().unwrap().rx_count.add(10);
    net0.metrics.get().write().unwrap().tap_read_fails.add(10);
    net0.metrics.get().write().unwrap().tap_write_fails.add(10);
    net0.metrics.get().write().unwrap().tx_bytes_count.add(10);
    net0.metrics.get().write().unwrap().tx_malformed_frames.add(10);
    net0.metrics.get().write().unwrap().tx_fails.add(10);
    net0.metrics.get().write().unwrap().tx_count.add(10);
    net0.metrics.get().write().unwrap().tx_packets_count.add(10);
    net0.metrics.get().write().unwrap().tx_partial_reads.add(10);
    net0.metrics.get().write().unwrap().tx_queue_event_count.add(10);
    net0.metrics.get().write().unwrap().tx_rate_limiter_event_count.add(10);
    net0.metrics.get().write().unwrap().tx_rate_limiter_throttled.add(10);
    net0.metrics.get().write().unwrap().tx_spoofed_mac_count.add(10);
    net1.metrics.get().write().unwrap().cfg_fails.add(10);
    net1.metrics.get().write().unwrap().mac_address_updates.add(10);
    net1.metrics.get().write().unwrap().no_rx_avail_buffer.add(10);
    net1.metrics.get().write().unwrap().no_tx_avail_buffer.add(10);
    net1.metrics.get().write().unwrap().event_fails.add(10);
    net1.metrics.get().write().unwrap().rx_queue_event_count.add(10);
    net1.metrics.get().write().unwrap().rx_event_rate_limiter_count.add(10);
    net1.metrics.get().write().unwrap().rx_partial_writes.add(10);
    net1.metrics.get().write().unwrap().rx_rate_limiter_throttled.add(10);
    net1.metrics.get().write().unwrap().rx_tap_event_count.add(10);
    net1.metrics.get().write().unwrap().rx_bytes_count.add(10);
    net1.metrics.get().write().unwrap().rx_packets_count.add(10);
    net1.metrics.get().write().unwrap().rx_fails.add(10);
    net1.metrics.get().write().unwrap().rx_count.add(10);
    net1.metrics.get().write().unwrap().tap_read_fails.add(10);
    net1.metrics.get().write().unwrap().tap_write_fails.add(10);
    net1.metrics.get().write().unwrap().tx_bytes_count.add(10);
    net1.metrics.get().write().unwrap().tx_malformed_frames.add(10);
    net1.metrics.get().write().unwrap().tx_fails.add(10);
    net1.metrics.get().write().unwrap().tx_count.add(10);
    net1.metrics.get().write().unwrap().tx_packets_count.add(10);
    net1.metrics.get().write().unwrap().tx_partial_reads.add(10);
    net1.metrics.get().write().unwrap().tx_queue_event_count.add(10);
    net1.metrics.get().write().unwrap().tx_rate_limiter_event_count.add(10);
    net1.metrics.get().write().unwrap().tx_rate_limiter_throttled.add(10);
    net1.metrics.get().write().unwrap().tx_spoofed_mac_count.add(10);
    let t1 = SystemTime::now();
    println!("Time taken to update metrics with proposal: {:?}", t1.duration_since(t0).unwrap());
// */

    let t0 = SystemTime::now();
    assert!(m.write().is_ok());
    let t1 = SystemTime::now();
    println!("Time taken to flush metrics with proposal: {:?}", t1.duration_since(t0).unwrap());
}

fn main(){
    let m = &METRICS;

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