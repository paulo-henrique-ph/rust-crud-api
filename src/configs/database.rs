use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::configs::environment::Env;
use crate::utils::error::AppError;

pub type Postgres = PostgresConnectionManager<NoTls>;

pub async fn init_postgres(env: &Env) -> Result<Pool<Postgres>, AppError> {
    let manager =
        PostgresConnectionManager::new_from_stringlike(&env.database_url, tokio_postgres::NoTls)?;

    let pool = Pool::builder()
        .min_idle(Some(3))
        .max_size(10)
        .connection_timeout(std::time::Duration::from_secs(3))
        .build(manager)
        .await?;

    Ok(pool)
}
