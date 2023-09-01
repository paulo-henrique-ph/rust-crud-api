use crate::configs::environment::Env;

use diesel::prelude::*;

pub async fn init_postgres(env: &Env) -> PgConnection {
    PgConnection::establish(&env.database_url).expect("failed to connect to postgres")
}
