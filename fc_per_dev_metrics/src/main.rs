mod metrics;
#[macro_use]
mod netdevice;
use crate::metrics::{METRICS, Metrics, FirecrackerMetrics};
use std::time::SystemTime;
use std::io::LineWriter;
use std::fs::File;
use crate::netdevice::Net;
use crate::netdevice::{get_net_metrics, NetDeviceMetricsAlloc};
use crate::metrics::IncMetric;

fn test_net_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
    let eth0_id = String::from("eth0");
    let eth1_id = String::from("eth1");
    Net::new(eth0_id.clone());
    Net::new(eth1_id.clone());
    let t0 = SystemTime::now();
// /*
    NET_METRICS!(&eth0_id,activate_fails.inc());
    NET_METRICS!(&eth0_id,cfg_fails.add(10));
    NET_METRICS!(&eth0_id,mac_address_updates.add(10));
    NET_METRICS!(&eth0_id,no_rx_avail_buffer.inc());
    NET_METRICS!(&eth0_id,no_tx_avail_buffer.inc());
    NET_METRICS!(&eth0_id,event_fails.inc());
    NET_METRICS!(&eth0_id,rx_queue_event_count.inc());
    NET_METRICS!(&eth0_id,rx_event_rate_limiter_count.inc());
    NET_METRICS!(&eth0_id,rx_partial_writes.add(10));
    NET_METRICS!(&eth0_id,rx_rate_limiter_throttled.add(10));
    NET_METRICS!(&eth0_id,rx_tap_event_count.add(10));
    NET_METRICS!(&eth0_id,rx_bytes_count.add(10));
    NET_METRICS!(&eth0_id,rx_packets_count.add(10));
    NET_METRICS!(&eth0_id,rx_fails.add(10));
    NET_METRICS!(&eth0_id,rx_count.add(10));
    NET_METRICS!(&eth0_id,tap_read_fails.add(10));
    NET_METRICS!(&eth0_id,tap_write_fails.add(10));
    NET_METRICS!(&eth0_id,tx_bytes_count.add(10));
    NET_METRICS!(&eth0_id,tx_malformed_frames.add(10));
    NET_METRICS!(&eth0_id,tx_fails.add(10));
    NET_METRICS!(&eth0_id,tx_count.add(10));
    NET_METRICS!(&eth0_id,tx_packets_count.add(10));
    NET_METRICS!(&eth0_id,tx_partial_reads.add(10));
    NET_METRICS!(&eth0_id,tx_queue_event_count.add(10));
    NET_METRICS!(&eth0_id,tx_rate_limiter_event_count.add(10));
    NET_METRICS!(&eth0_id,tx_rate_limiter_throttled.add(10));
    NET_METRICS!(&eth0_id,tx_spoofed_mac_count.add(10));

    NET_METRICS!(&eth1_id, activate_fails.inc());
    NET_METRICS!(&eth1_id, cfg_fails.add(10));
    NET_METRICS!(&eth1_id, mac_address_updates.add(10));
    NET_METRICS!(&eth1_id, no_rx_avail_buffer.add(10));
    NET_METRICS!(&eth1_id, no_tx_avail_buffer.add(10));
    NET_METRICS!(&eth1_id, event_fails.add(10));
    NET_METRICS!(&eth1_id, rx_queue_event_count.add(10));
    NET_METRICS!(&eth1_id, rx_event_rate_limiter_count.add(10));
    NET_METRICS!(&eth1_id, rx_partial_writes.add(10));
    NET_METRICS!(&eth1_id, rx_rate_limiter_throttled.add(10));
    NET_METRICS!(&eth1_id, rx_tap_event_count.add(10));
    NET_METRICS!(&eth1_id, rx_bytes_count.add(10));
    NET_METRICS!(&eth1_id, rx_packets_count.add(10));
    NET_METRICS!(&eth1_id, rx_fails.add(10));
    NET_METRICS!(&eth1_id, rx_count.add(10));
    NET_METRICS!(&eth1_id, tap_read_fails.add(10));
    NET_METRICS!(&eth1_id, tap_write_fails.add(10));
    NET_METRICS!(&eth1_id, tx_bytes_count.add(10));
    NET_METRICS!(&eth1_id, tx_malformed_frames.add(10));
    NET_METRICS!(&eth1_id, tx_fails.add(10));
    NET_METRICS!(&eth1_id, tx_count.add(10));
    NET_METRICS!(&eth1_id, tx_packets_count.add(10));
    NET_METRICS!(&eth1_id, tx_partial_reads.add(10));
    NET_METRICS!(&eth1_id, tx_queue_event_count.add(10));
    NET_METRICS!(&eth1_id, tx_rate_limiter_event_count.add(10));
    NET_METRICS!(&eth1_id, tx_rate_limiter_throttled.add(10));
    NET_METRICS!(&eth1_id, tx_spoofed_mac_count.add(10));
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