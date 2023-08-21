use opentelemetry_api::metrics::Unit;
use opentelemetry_api::{metrics::MeterProvider as _, KeyValue};
use opentelemetry_sdk::metrics::{MeterProvider, PeriodicReader};
use opentelemetry_sdk::{runtime, Resource};
use std::error::Error;

fn init_meter_provider() -> MeterProvider {
    let exporter = opentelemetry_stdout::MetricsExporter::default();
    let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();
    MeterProvider::builder()
        .with_reader(reader)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            "metrics-basic-example",
        )]))
        .build()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Initialize the MeterProvider with the stdout Exporter.
    let meter_provider = init_meter_provider();

    // Create a meter from the above MeterProvider.
    let meter = meter_provider.meter("fc_meter");

    let mycounter1 = meter.u64_counter("NetDeviceMetrics").with_unit(Unit::new("Bytes")).init();
    mycounter1.add(100, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev0", "rx_bytes_count")].as_ref());
    mycounter1.add(101, [KeyValue::new("dev0", "tx_bytes_count")].as_ref());
    let mycounter2 = meter.u64_counter("vsock").with_unit(Unit::new("Bytes")).init();
    mycounter2.add(4, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev0", "rx_bytes_count")].as_ref());
    mycounter2.add(5, [KeyValue::new("dev0", "tx_bytes_count")].as_ref());

    mycounter1.add(8, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev1", "rx_bytes_count")].as_ref());
    mycounter1.add(9, [KeyValue::new("dev1", "tx_bytes_count")].as_ref());
    mycounter2.add(8, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev1", "rx_bytes_count")].as_ref());
    mycounter2.add(9, [KeyValue::new("dev1", "tx_bytes_count")].as_ref());

    // Metrics are exported by default every 30 seconds when using stdout exporter,
    // however shutting down the MeterProvider here instantly flushes
    // the metrics, instead of waiting for the 30 sec interval.
    meter_provider.shutdown()?;
    Ok(())
}
