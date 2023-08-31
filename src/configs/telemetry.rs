use opentelemetry::sdk::metrics::reader::{DefaultAggregationSelector, DefaultTemporalitySelector};
use opentelemetry::sdk::metrics::{Aggregation, Instrument, MeterProvider, PeriodicReader, Stream};
use opentelemetry::sdk::trace::{config, RandomIdGenerator, Sampler, Tracer};
use opentelemetry::sdk::Resource;
use opentelemetry::{global, runtime, Key, KeyValue};
use opentelemetry_otlp::{Protocol, TonicExporterBuilder, WithExportConfig};
use tonic::metadata::{Ascii, MetadataKey, MetadataMap, MetadataValue};
use tracing::Level;
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::new(vec![
        KeyValue::new("service.name", "rust-crud-api"),
        KeyValue::new("service.type", "api"),
        KeyValue::new("environment", "development"),
        KeyValue::new("library.language", "rust"),
    ])
}

// Construct MeterProvider for MetricsLayer
fn init_meter_provider() -> MeterProvider {
    // Currently we could not access MeterProviderBuilder from opentelemetry_otlp
    // However to customize View we need MeterBuilder, so manually construct.
    let exporter = opentelemetry_otlp::MetricsExporter::new(
        TonicExporterBuilder::default(),
        Box::new(DefaultTemporalitySelector::new()),
        Box::new(DefaultAggregationSelector::new()),
    )
    .unwrap();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    // For debugging in development
    let stdout_reader = PeriodicReader::builder(
        opentelemetry_stdout::MetricsExporter::default(),
        runtime::Tokio,
    )
    .build();

    // Rename foo metrics to foo_named and drop key_2 attribute
    let view_foo = |instrument: &Instrument| -> Option<Stream> {
        if instrument.name == "foo" {
            Some(
                Stream::new()
                    .name("foo_named")
                    .allowed_attribute_keys([Key::from("key_1")]),
            )
        } else {
            None
        }
    };

    // Set Custom histogram boundaries for baz metrics
    let view_baz = |instrument: &Instrument| -> Option<Stream> {
        if instrument.name == "baz" {
            Some(
                Stream::new()
                    .name("baz")
                    .aggregation(Aggregation::ExplicitBucketHistogram {
                        boundaries: vec![0.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0],
                        record_min_max: true,
                    }),
            )
        } else {
            None
        }
    };

    let meter_provider = MeterProvider::builder()
        .with_resource(resource())
        .with_reader(reader)
        .with_reader(stdout_reader)
        .with_view(view_foo)
        .with_view(view_baz)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

// Construct Tracer for OpenTelemetryLayer
fn init_tracer() -> Tracer {
    let config = config()
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

    let mut map = MetadataMap::with_capacity(3);

    let key = MetadataKey::<Ascii>::from_static("api-key");
    let value = MetadataValue::<Ascii>::from_static("k");

    map.insert(key, value);

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

    tracer
}

// Initialize tracing-subscriber and return OtelGuard for opentelemetry-related termination processing
pub fn init_tracing_subscriber() -> OtelGuard {
    let meter_provider = init_meter_provider();

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(init_tracer()))
        .init();

    OtelGuard { meter_provider }
}

pub struct OtelGuard {
    meter_provider: MeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
        opentelemetry::global::shutdown_tracer_provider();
    }
}
