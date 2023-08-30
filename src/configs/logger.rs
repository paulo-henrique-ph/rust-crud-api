use opentelemetry::{
    global,
    runtime::TokioCurrentThread,
    sdk::{
        propagation::TraceContextPropagator,
    },
};
use tracing_actix_web::{DefaultRootSpanBuilder, TracingLogger};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn with_logger() -> TracingLogger<DefaultRootSpanBuilder> {
    TracingLogger::default()
}

// Initialize OpenTelemetry
pub fn init_telemetry() {
    global::set_text_map_propagator(TraceContextPropagator::new());

    // let tracer = opentelemetry_jaeger::new_pipeline().with_service_name("actix-web-opentelemetry").install_batch(TokioCurrentThread).expect("pipeline install failure");

    // Filter based on level - trace, debug, info, warn, error
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default().with(env_filter).with(telemetry);

    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed")
}

// End OpenTelemetry gracefully
pub fn end_telemetry() {
    global::shutdown_tracer_provider();
}