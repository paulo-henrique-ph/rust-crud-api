use std::sync::Arc;

use crate::dto::car::CarDto;
use crate::models::car::Car;
use crate::repositories::car::CarRepository;
pub struct CarService {
    pub car_repository: Arc<dyn CarRepository>,
}

impl CarService {
    pub fn new(car_repository: Arc<dyn CarRepository>) -> Self {
        Self { car_repository }
    }

    pub async fn get_cars(&self) -> Result<Vec<Car>, Error> {
        self.car_repository.get_cars().await
    }

    pub async fn get_car(&self, id: i32) -> Result<Car, Error> {
        self.car_repository.get_car(id).await
    }

    pub async fn create_car(&self, car: CarDto) -> Result<Car, Error> {
        self.car_repository.create_car(car).await
    }

    pub async fn update_car(&self, id: i32, car: CarDto) -> Result<Car, Error> {
        self.car_repository.update_car(id, car).await
    }

    pub async fn delete_car(&self, id: i32) -> Result<(), Error> {
        self.car_repository.delete_car(id).await
    }
}
