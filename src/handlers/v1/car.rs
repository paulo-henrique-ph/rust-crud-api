use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::dto::car::CarDto;

pub async fn create_handler(req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json("OK")
}

pub async fn delete_handler(path: web::Path<i32>) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json("OK")
}

pub async fn get_handler(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::build(StatusCode::OK).json("OK")
}

pub async fn update_handler(path: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    HttpResponse::build(StatusCode::OK).json("OK")
}
