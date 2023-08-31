#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("postgres pool error: {0}")]
    PostgresPool(#[from] bb8_postgres::bb8::RunError<tokio_postgres::Error>),

    #[error("postgres error: {0}")]
    Postgres(#[from] tokio_postgres::Error),
}
