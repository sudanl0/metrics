mod metrics;
mod netdevice;
use crate::metrics::{METRICS, Metrics, FirecrackerMetrics};
use std::time::SystemTime;
use std::io::LineWriter;
use std::fs::File;
use crate::netdevice::{Net, get_net_metrics};
use crate::metrics::IncMetric;

fn test_net_metrics(m: &Metrics<FirecrackerMetrics, LineWriter<File>>){
    let eth0_id = String::from("eth0");
    let eth1_id = String::from("eth1");
    Net::new(eth0_id.clone());
    Net::new(eth1_id.clone());
    let t0 = SystemTime::now();
// /*
    get_net_metrics().metrics.get(&eth0_id).unwrap().activate_fails.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().cfg_fails.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().mac_address_updates.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().no_rx_avail_buffer.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().no_tx_avail_buffer.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().event_fails.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_queue_event_count.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_event_rate_limiter_count.inc();
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_partial_writes.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_rate_limiter_throttled.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_tap_event_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_bytes_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_packets_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_fails.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().rx_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tap_read_fails.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tap_write_fails.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_bytes_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_malformed_frames.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_fails.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_packets_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_partial_reads.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_queue_event_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_rate_limiter_event_count.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_rate_limiter_throttled.add(10);
    get_net_metrics().metrics.get(&eth0_id).unwrap().tx_spoofed_mac_count.add(10);

    get_net_metrics().metrics.get(&eth1_id).unwrap().activate_fails.inc();
    get_net_metrics().metrics.get(&eth1_id).unwrap().cfg_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().mac_address_updates.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().no_rx_avail_buffer.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().no_tx_avail_buffer.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().event_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_queue_event_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_event_rate_limiter_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_partial_writes.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_rate_limiter_throttled.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_tap_event_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_bytes_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_packets_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().rx_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tap_read_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tap_write_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_bytes_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_malformed_frames.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_fails.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_packets_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_partial_reads.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_queue_event_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_rate_limiter_event_count.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_rate_limiter_throttled.add(10);
    get_net_metrics().metrics.get(&eth1_id).unwrap().tx_spoofed_mac_count.add(10);
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