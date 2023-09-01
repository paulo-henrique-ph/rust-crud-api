use crate::configs::database;
use crate::dto::car::CarDto;
use crate::entities;
use diesel::{QueryDsl, RunQueryDsl};
use tokio_postgres::Error;
use tonic::async_trait;

use crate::models::car::Car;

pub async fn get_cars() -> Result<Vec<Car>, Error> {
    let connection = database::init_postgres(&self.env);
    entities::schema::cars::table.load::<Vec<Car>>(&connection)
}
async fn get_car(id: i32) -> Result<Car, Error> {
    let connection = database::init_postgres(&self.env);
    entities::schema::cars::table
        .filter(entities::schema::cars::id.eq(id))
        .first::<Car>(&connection)
}
async fn create_car(car: CarDto) -> Result<Car, Error> {
    let connection = database::init_postgres(&self.env);
    diesel::insert_into(entities::schema::cars::table)
        .values(&car)
        .get_result(&connection)
}
async fn update_car(id: i32, car: CarDto) -> Result<Car, Error> {
    let connection = database::init_postgres(&self.env);
    diesel::update(entities::schema::cars::table.find(id))
        .set(&car)
        .get_result(&connection)
}
async fn delete_car(id: i32) -> Result<(), Error> {
    let connection = database::init_postgres(&self.env);
    diesel::delete(entities::schema::cars::table.find(id)).execute(&connection)
}

/*#[async_trait]
pub trait CarRepository {
    async fn get_cars(&self) -> Result<Vec<Car>, Error>;
    async fn get_car(&self, id: i32) -> Result<Car, Error>;
    async fn create_car(&self, car: CarDto) -> Result<Car, Error>;
    async fn update_car(&self, id: i32, car: CarDto) -> Result<Car, Error>;
    async fn delete_car(&self, id: i32) -> Result<(), Error>;
}
*/
