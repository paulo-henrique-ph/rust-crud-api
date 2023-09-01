use log::{error, info};
use opentelemetry::sdk::propagation::{
    BaggagePropagator, TextMapCompositePropagator, TraceContextPropagator,
};
use opentelemetry::sdk::trace::{RandomIdGenerator, Sampler};
use opentelemetry::sdk::{trace, Resource};
use opentelemetry::{global, runtime, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use tonic::metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

use crate::configs::environment::Env;

pub fn setup(env: &Env) -> WorkerGuard {
    let mut map = MetadataMap::with_capacity(3);

    let key = MetadataKey::<Ascii>::from_static("api-key");
    let value =
        MetadataValue::<Ascii>::try_from(&env.new_relic_key).expect("Couldn't get new relic key");

    map.insert(key, value);

    let config = trace::config()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_max_events_per_span(64)
        .with_max_attributes_per_span(16)
        .with_resource(Resource::new(vec![
            KeyValue::new("service.name", "rust-crud-api"),
            KeyValue::new("service.type", "api"),
            KeyValue::new("environment", "development"),
            KeyValue::new("library.language", "rust"),
        ]));

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("https://otlp.nr-data.net:4317")
        .with_protocol(Protocol::Grpc)
        .with_timeout(std::time::Duration::from_secs(3))
        .with_metadata(map);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(config)
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .unwrap();

    let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    Registry::default()
        .with(EnvFilter::new("info"))
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(false)
                .with_span_list(false)
                .with_file(false)
                .with_line_number(false)
                .with_writer(writer),
        )
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    opentelemetry::global::set_error_handler(|error| {
        error!("OpenTelemetry error occurred: {:#}", anyhow::anyhow!(error),);
    })
    .expect("to be able to set error handler");

    info!("traces::setup tracer installed");
    guard
}
