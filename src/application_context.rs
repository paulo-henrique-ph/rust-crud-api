use crate::configs;
use crate::configs::database::Postgres;
use bb8_postgres::bb8::Pool;
use std::sync::Arc;

use crate::configs::environment::Env;
use crate::utils::error::AppError;

pub type SharedApplicationContext = Arc<ApplicationContext>;

pub struct ApplicationContext {
    postgres: Pool<Postgres>,
}

impl ApplicationContext {
    pub async fn autowire(env: &Env) -> Result<SharedApplicationContext, AppError> {
        let postgres = configs::database::init_postgres(env).await?;

        Ok::<Arc<Self>, AppError>(Arc::new(Self { postgres }))
    }
}
