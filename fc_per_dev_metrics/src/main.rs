mod metrics;
mod netdevice;
use crate::metrics::METRICS;
use crate::metrics::IncMetricPerDev;
use crate::metrics::PerDevMetrics;
use crate::metrics::IncMetric;
use crate::netdevice::Net;
use crate::netdevice::NetDeviceMetricsFns;
use std::time::SystemTime;

fn main(){
    use std::io::LineWriter;
    use std::fs::File;
    let m = &METRICS;

    let res = m.write();
    assert!(res.is_ok() && !res.unwrap());

    let f = File::create("./metrics.json").expect("Failed to create temporary metrics file");
    assert!(m.init(LineWriter::new(f)).is_ok());
    assert!(m.write().is_ok());

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// BTreeMap in PerDevBlockDeviceMetrics
    //////////////////////////////////////////////////////////////////////////////////////////

    let t0 = SystemTime::now();

    METRICS.block.add(&String::from("block0"), "activate_fails", 10);
    METRICS.block.add(&String::from("block1"), "activate_fails", 10);
    assert!(m.write().is_ok());
    let t1 = SystemTime::now();
    println!("{:?}", t1.duration_since(t0).unwrap());

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// BTreeMap in SharedIncMetricPerDev
    //////////////////////////////////////////////////////////////////////////////////////////

    let t0 = SystemTime::now();
    METRICS.vsock.activate_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.activate_fails.add(&String::from("vsock1"), 20);
    assert!(m.write().is_ok());
    // println!("{:?}", METRICS.vsock);
    let t1 = SystemTime::now();
    println!("{:?}", t1.duration_since(t0).unwrap());

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// NetDeviceMetrics moved to Net as NetPerDeviceMetrics with no BTreeMap
    //////////////////////////////////////////////////////////////////////////////////////////
    let net0 = Net::new(String::from("net0"));
    let net1 = Net::new(String::from("net1"));
    net0.metrics.activate_fails.add(20);
    net1.metrics.activate_fails.add(40);

    let t0 = SystemTime::now();
    assert!(m.write().is_ok());
    match serde_json::to_string_pretty(&net0){
        Ok(net_metrics_serbuf) => {
            assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
        }
        Err(err) => println!("{}", err.to_string())
    }
    match serde_json::to_string_pretty(&net1){
        Ok(net_metrics_serbuf) => {
            assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
        }
        Err(err) => println!("{}", err.to_string())
    }
    let t1 = SystemTime::now();
    println!("{:?}", t1.duration_since(t0).unwrap());

    //////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// NetDeviceMetrics moved to Net as NetPerDeviceMetrics with
    /////////// aggregate using trait and no BTreeMap
    //////////////////////////////////////////////////////////////////////////////////////////

    let t0 = SystemTime::now();
    let net0 = Net::new(String::from("net0"));
    let net1 = Net::new(String::from("net1"));
    net0.metrics.activate_fails_add(20);
    net1.metrics.activate_fails_add(40);
    assert!(m.write().is_ok());
    match serde_json::to_string_pretty(&net0){
        Ok(net_metrics_serbuf) => {
            assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
        }
        Err(err) => println!("{}", err.to_string())
    }
    match serde_json::to_string_pretty(&net1){
        Ok(net_metrics_serbuf) => {
            assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
        }
        Err(err) => println!("{}", err.to_string())
    }
    let t1 = SystemTime::now();
    println!("{:?}", t1.duration_since(t0).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::metrics::{METRICS, METRICS1, METRICS2, METRICS3};
    use std::time::SystemTime;
    use std::io::LineWriter;
    use std::fs::File;
    
    #[test]
    fn test_btree_mapin_per_dev_block_device_metrics() {
        // average 100usecs on my machine
        use crate::metrics::PerDevMetrics;
        let m = &METRICS;
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());
        
        let f = File::create("./block_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());

        let t0 = SystemTime::now();
        METRICS.block.add(&String::from("block0"), "activate_fails", 10);
        METRICS.block.add(&String::from("block1"), "activate_fails", 10);
        assert!(m.write().is_ok());
        let t1 = SystemTime::now();

        println!("[test_btree_mapin_per_dev_block_device_metrics] : {:?}", t1.duration_since(t0).unwrap());
    }

    #[test]
    fn test_btree_mapin_shared_inc_metric_per_dev() {
        // average 80usecs on my machine
        use crate::metrics::IncMetricPerDev;
        let m = &METRICS1;
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());
        
        let f = File::create("./vsock_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());

        let t0 = SystemTime::now();
        METRICS.vsock.activate_fails.add(&String::from("vsock0"), 10);
        METRICS.vsock.activate_fails.add(&String::from("vsock1"), 20);
        assert!(m.write().is_ok());
        let t1 = SystemTime::now();

        println!("[test_btree_mapin_shared_inc_metric_per_dev] : {:?}", t1.duration_since(t0).unwrap());
    }

    #[test]
    fn test_moved_metrics() {
        // average 100usecs on my machine but this doesn't print aggregate
        use crate::netdevice::Net;
        use crate::metrics::IncMetric;
        let m = &METRICS2;
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());
        
        let f = File::create("./net_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());
        
        let net0 = Net::new(String::from("net0"));
        let net1 = Net::new(String::from("net1"));
        
        let t0 = SystemTime::now();
        net0.metrics.activate_fails.add(20);
        net1.metrics.activate_fails.add(40);
        assert!(m.write().is_ok());
        match serde_json::to_string_pretty(&net0){
            Ok(net_metrics_serbuf) => {
                assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
            }
            Err(err) => println!("{}", err.to_string())
        }
        match serde_json::to_string_pretty(&net1){
            Ok(net_metrics_serbuf) => {
                assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
            }
            Err(err) => println!("{}", err.to_string())
        }
        let t1 = SystemTime::now();
        println!("[test_moved_metrics] : {:?}", t1.duration_since(t0).unwrap());
    }

    #[test]
    fn test_aggregate() {
        // average 105usecs on my machine
        use crate::netdevice::Net;
        use crate::netdevice::NetDeviceMetricsFns;
        use std::time::SystemTime;
        use std::io::LineWriter;
        use std::fs::File;
        let m = &METRICS3;
        
        let res = m.write();
        assert!(res.is_ok() && !res.unwrap());

        let f = File::create("./neta_metrics.json").expect("Failed to create temporary metrics file");
        assert!(m.init(LineWriter::new(f)).is_ok());
        assert!(m.write().is_ok());

        let net0 = Net::new(String::from("net0"));
        let net1 = Net::new(String::from("net1"));

        let t0 = SystemTime::now();
        net0.metrics.activate_fails_add(20);
        net1.metrics.activate_fails_add(40);
        assert!(m.write().is_ok());
        match serde_json::to_string_pretty(&net0){
            Ok(net_metrics_serbuf) => {
                assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
            }
            Err(err) => println!("{}", err.to_string())
        }
        match serde_json::to_string_pretty(&net1){
            Ok(net_metrics_serbuf) => {
                assert!(m.write_devmetrics(net_metrics_serbuf).is_ok());
            }
            Err(err) => println!("{}", err.to_string())
        }
        let t1 = SystemTime::now();

        println!("[test_aggregate] : {:?}", t1.duration_since(t0).unwrap());
    }
}