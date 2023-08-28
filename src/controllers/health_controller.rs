use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/health")]
pub async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}