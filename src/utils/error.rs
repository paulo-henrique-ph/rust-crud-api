use actix_web::{
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,

    #[display(fmt = "Forbidden")]
    Forbidden,

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Conflict: {}", _0)]
    Conflict(String),

    #[display(fmt = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(String),

    #[display(fmt = "Service Unavailable")]
    ServiceUnavailable,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            AppError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            AppError::Unauthorized => HttpResponse::Unauthorized().finish(),
            AppError::Forbidden => HttpResponse::Forbidden().finish(),
            AppError::NotFound(ref message) => HttpResponse::NotFound().json(message),
            AppError::Conflict(ref message) => HttpResponse::Conflict().json(message),
            AppError::UnprocessableEntity(ref message) => {
                HttpResponse::build(StatusCode::UNPROCESSABLE_ENTITY).json(message)
            }
            AppError::ServiceUnavailable => {
                HttpResponse::ServiceUnavailable().finish()
            }
        }
    }
}