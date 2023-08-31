use actix_web::http::StatusCode;
use actix_web::HttpResponse;

pub async fn create_handler() -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json("OK")
}

pub async fn delete_handler() -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json("Ok")
}
