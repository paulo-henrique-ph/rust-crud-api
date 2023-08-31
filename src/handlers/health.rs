use actix_web::HttpResponse;
use tracing::info;

#[tracing::instrument(
    name = "Health check handler",
    fields(
        otel.kind = "server",
        http.request.method = "GET",
        url.path = "/health"
    )
)]
pub async fn handler() -> HttpResponse {
    info!("handling /health");

    tracing::info!(histogram.baz = 10, "histogram example",);

    HttpResponse::Ok().finish()
}
