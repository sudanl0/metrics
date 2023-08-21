use once_cell::sync::Lazy;
use opentelemetry_api::global;
use opentelemetry_api::{
    metrics,
    Key, KeyValue,
};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{metrics::MeterProvider, runtime, Resource};
use std::error::Error;
use opentelemetry_api::metrics::Unit;

fn init_metrics() -> metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "basic-otlp-metrics-example",
        )]))
        .build()
}

const LEMONS_KEY: Key = Key::from_static_str("lemons");

static COMMON_ATTRIBUTES: Lazy<[KeyValue; 4]> = Lazy::new(|| {
    [
        LEMONS_KEY.i64(10),
        KeyValue::new("A", "1"),
        KeyValue::new("B", "2"),
        KeyValue::new("C", "3"),
    ]
});

fn with_dev_id(device: &str, metrics: &str, dev_id: u32) -> String {
	format!("{device}_{dev_id}::{metrics}")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let meter_provider = init_metrics()?;

    let meter = global::meter("fc-meter");


    let mycounter1 = meter.u64_counter("NetDeviceMetrics").with_unit(Unit::new("Bytes")).init();
    let mycounter2 = meter.u64_counter("vsock").with_unit(Unit::new("Bytes")).init();

    mycounter1.add(100, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev0", "rx_bytes_count")].as_ref());
    mycounter1.add(101, [KeyValue::new("dev0", "tx_bytes_count")].as_ref());
    mycounter2.add(4, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev0", "rx_bytes_count")].as_ref());
    mycounter2.add(5, [KeyValue::new("dev0", "tx_bytes_count")].as_ref());

    mycounter1.add(8, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev1", "rx_bytes_count")].as_ref());
    mycounter1.add(9, [KeyValue::new("dev1", "tx_bytes_count")].as_ref());
    mycounter2.add(8, [KeyValue::new("total", "rx_bytes_count"), KeyValue::new("dev1", "rx_bytes_count")].as_ref());
    mycounter2.add(9, [KeyValue::new("dev1", "tx_bytes_count")].as_ref());
/*
	let mycounter1 = meter.u64_counter("NetDeviceMetrics").with_unit(Unit::new("Bytes")).init();
	mycounter1.add(100, [KeyValue::new("net0", "rx_bytes_count")].as_ref());
	let mycounter2 = meter.u64_counter("vsock").with_unit(Unit::new("Bytes")).init();
	mycounter2.add(4, [KeyValue::new("vsock0", "rx_bytes_count")].as_ref());

	mycounter1.add(8, [KeyValue::new("net1", "rx_bytes_count")].as_ref());
	mycounter2.add(8, [KeyValue::new("vsock1", "rx_bytes_count")].as_ref());
*/
    meter_provider.shutdown()?;

    Ok(())
}
