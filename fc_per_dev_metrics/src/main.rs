mod metrics;
mod netdevice;
use crate::metrics::{METRICS, IncMetric, IncMetricPerDev, PerDevMetrics};
use crate::netdevice::Net;

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
    METRICS.block.add(&String::from("block0"), "activate_fails", 10);
    METRICS.block.add(&String::from("block1"), "activate_fails", 10);
    assert!(m.write().is_ok());

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// BTreeMap in SharedIncMetricPerDev
    //////////////////////////////////////////////////////////////////////////////////////////
    METRICS.vsock.activate_fails.add(&String::from("vsock0"), 10);
    METRICS.vsock.activate_fails.add(&String::from("vsock1"), 20);
    assert!(m.write().is_ok());
    // println!("{:?}", METRICS.vsock);

    //////////////////////////////////////////////////////////////////////////////////////////
    /////////// NetDeviceMetrics moved to Net as NetPerDeviceMetrics with no BTreeMap
    //////////////////////////////////////////////////////////////////////////////////////////
    let net0 = Net::new(String::from("net0"));
    net0.metrics.activate_fails.add(20);
    let net1 = Net::new(String::from("net1"));
    net1.metrics.activate_fails.add(40);
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
    //////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////
}

