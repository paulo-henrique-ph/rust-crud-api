/*use actix_web::web::{self};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Error, Pool,
                   PooledConnection, PoolError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

// Get DB Connection pool
pub fn get_connection_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}

pub trait PgConnectionHandler {
    // Run operation (closure) by passing in mutable ref of an actual connection
    fn run_with_conn<F, D>(&mut self, func: F) -> Result<D, Error>
        where
            F: Fn(&mut PgConnection) -> Result<D, Error>;
}

impl PgConnectionHandler for Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
    fn run_with_conn<F, D>(&mut self, func: F) -> Result<D, Error>
        where
            F: Fn(&mut PgConnection) -> Result<D, Error>,
    {
        match self.as_deref_mut() {
            Ok(conn) => func(conn),
            Err(e) => Err(e.clone()),
        }
    }
}

pub trait PooledConnectionHandler {
    fn run_with_conn<F, D>(self, func: F) -> Result<D, PoolError>
        where
            F: Fn(&mut PgConnection) -> Result<D, Error>;
}

impl PooledConnectionHandler for web::Data<PgPool> {
    // Get a pooled connection and run operation (closure) with the connection acquired
    fn run_with_conn<F, D>(self, func: F) -> Result<D, PoolError> where F: Fn(&mut PgConnection) -> Result<D, Error> {
        self.get().unwrap().run_with_conn(func)
    }
}*/