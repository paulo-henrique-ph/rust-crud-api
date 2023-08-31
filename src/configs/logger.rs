use log::info;
use opentelemetry::sdk::propagation::{
    BaggagePropagator, TextMapCompositePropagator, TraceContextPropagator,
};
use opentelemetry::sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::{global, runtime, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tonic::metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

pub fn setup() {
    let mut map = MetadataMap::with_capacity(3);

    let key = MetadataKey::<Ascii>::from_static("api-key");
    let value = MetadataValue::<Ascii>::from_static("key");

    map.insert(key, value);

    let config = trace::config()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(Resource::new(vec![
            KeyValue::new("service.name", "TEST-API"),
            KeyValue::new("service.type", "api"),
            KeyValue::new("environment", "development"),
            KeyValue::new("library.language", "rust"),
        ]));

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("https://otlp.nr-data.net")
        .with_protocol(Protocol::Grpc)
        .with_timeout(std::time::Duration::from_secs(3))
        .with_metadata(map);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(config)
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .unwrap();

    Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(false)
                .with_span_list(false)
                .with_file(false)
                .with_line_number(false),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    info!("traces::setup tracer installed");
}
