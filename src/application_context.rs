use crate::configs;
use diesel::prelude::*;
use std::sync::Arc;

use crate::configs::environment::Env;
use crate::services::car::CarService;
use crate::utils::error::AppError;

pub type SharedApplicationContext = Arc<ApplicationContext>;

pub struct ApplicationContext {
    postgres: PgConnection,
}

impl ApplicationContext {
    pub async fn autowire(env: &Env) -> Result<SharedApplicationContext, AppError> {
        let postgres = configs::database::init_postgres(env).await?;
        // let car_service = CarService::new(postgres.clone());

        Ok::<Arc<Self>, AppError>(Arc::new(Self { postgres }))
    }
}
