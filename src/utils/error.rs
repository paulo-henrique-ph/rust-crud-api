#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("postgres pool error: {0}")]
    PostgresPool(#[from] bb8_postgres::bb8::RunError<tokio_postgres::Error>),

    #[error("postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("internal server error")]
    InternalServerError,

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("forbidden")]
    Forbidden,

    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("service unavailable")]
    ServiceUnavailable,
}
