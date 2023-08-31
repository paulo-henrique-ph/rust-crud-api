use opentelemetry::global;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

// Initialize OpenTelemetry
pub fn init_telemetry() -> WorkerGuard {
    let (non_blocking, guard) = NonBlockingBuilder::default()
        .lossy(false)
        .finish(std::io::stdout());

    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .install_simple()
        .unwrap();

    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry().with(otel_layer).init();

    guard
}

// End OpenTelemetry gracefully
pub fn end_telemetry() {
    global::shutdown_tracer_provider();
}
