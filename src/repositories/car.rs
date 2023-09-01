use async_trait::async_trait;

use crate::models::car::Car;

#[async_trait]
pub trait CarRepository {
    async fn get_cars(&self) -> Result<Vec<Car>, Error>;
    async fn get_car(&self, id: i32) -> Result<Car, Error>;
    async fn create_car(&self, car: Car) -> Result<Car, Error>;
    async fn update_car(&self, id: i32, car: Car) -> Result<Car, Error>;
    async fn delete_car(&self, id: i32) -> Result<(), Error>;
}
