
// use std::fmt::format;

use metrics::Unit;

/*
Note:
	- metrics::absolute_counter doesn't work but counter.absolute does.
 */

fn with_dev_id(device: &str, metrics: &str, dev_id: u32) -> String {
	format!("{device}_{dev_id}::{metrics}")
}

fn main() {
	let metrics = metrics_cloudwatch_embedded::Builder::new()
		.cloudwatch_namespace("MyApplication")
		.init()
		.unwrap();

	let dev_id = 0;
	// let net_dev_name = "NetDeviceMetrics::rx_bytes_count", dev_id);
	metrics::describe_counter!(with_dev_id("NetDeviceMetrics", "rx_bytes_count", dev_id), Unit::Bytes, "Number of bytes received");
	metrics::counter!(with_dev_id("NetDeviceMetrics", "rx_bytes_count", dev_id), 100, "Firecracker" => "net");

	metrics::describe_counter!(with_dev_id("vsock", "rx_bytes_count", dev_id), Unit::Bytes, "Number of bytes received");
	metrics::counter!(with_dev_id("vsock", "rx_bytes_count", dev_id), 100, "Firecracker" => "block");

	let _ = metrics
		.set_property("RequestId", "ABC123")
		.flush(std::io::stdout());
	metrics::counter!(with_dev_id("NetDeviceMetrics", "rx_bytes_count", dev_id), 250, "Firecracker" => "net");
	metrics::counter!(with_dev_id("vsock", "rx_bytes_count", dev_id), 150, "Firecracker" => "block");

	let _ = metrics
		.set_property("RequestId", "ABC123")
		.flush(std::io::stdout());
}
