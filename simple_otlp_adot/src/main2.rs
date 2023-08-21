use once_cell::sync::Lazy;
use opentelemetry_api::global;
use opentelemetry_api::{
    metrics,
    Key, KeyValue,
};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{metrics::MeterProvider, runtime, Resource};
use std::error::Error;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let meter_provider = init_metrics()?;

    let meter = global::meter("basic_meter");

	let mycounter1 = meter.f64_up_down_counter("mycounter1").init();
	mycounter1.add(4.1, [KeyValue::new("SomeKey", "1")].as_ref());
	mycounter1.add(40.0, [KeyValue::new("AnotherKey", "2")].as_ref());
	// mycounter1.add(-50.5);
    // let gauge = meter
    //     .f64_observable_gauge("basic_gauge")
    //     .with_description("A gauge set to 1.0")
    //     .init();

    // meter.register_callback(&[gauge.as_any()], move |observer| {
    //     observer.observe_f64(&gauge, 1.0, COMMON_ATTRIBUTES.as_ref())
    // })?;

    // let histogram = meter.f64_histogram("basic_histogram").init();
    // histogram.record(5.5, COMMON_ATTRIBUTES.as_ref());

    meter_provider.shutdown()?;

    Ok(())
}
