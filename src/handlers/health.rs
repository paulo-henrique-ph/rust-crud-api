use actix_web::HttpResponse;

#[tracing::instrument(
    name = "Health check handler",
    fields(
        otel.kind = "server",
        http.request.method = "GET",
        url.path = "/health"
    )
)]
pub async fn handler() -> HttpResponse {
    tracing::info!("handling request");
    HttpResponse::Ok().json("ok")
}
