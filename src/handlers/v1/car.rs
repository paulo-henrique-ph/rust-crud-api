use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};

use crate::dto::car::CarDto;
use crate::services::car::CarService;

pub async fn create_handler(req: HttpRequest) -> HttpResponse {
    let car = req
        .json::<CarDto>()
        .await
        .expect("failed to deserialize request body");
    let car_response = CarService::create_car(car).await;
    match car_response {
        Ok(response) => HttpResponse::build(StatusCode::OK).json(response),
        Err(error) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error),
    }
}

pub async fn delete_handler(path: web::Path<i32>) -> HttpResponse {
    let id = path.into_inner();
    let car_response = CarService::delete_car(id).await;
    match car_response {
        Ok(response) => HttpResponse::build(StatusCode::OK).json(response),
        Err(error) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error),
    }
}

pub async fn get_handler(path: web::Path<Option<i32>>) -> HttpResponse {
    let id = path.into_inner();
    match id {
        None => {
            let cars_response = CarService::get_cars().await;
            match cars_response {
                Ok(response) => HttpResponse::build(StatusCode::OK).json(response),
                Err(error) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error),
            }
        }
        Some(id) => {
            let car_response = CarService::get_car(id).await;
            match car_response {
                Ok(response) => HttpResponse::build(StatusCode::OK).json(response),
                Err(error) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error),
            }
        }
    }
}

pub async fn update_handler(path: web::Path<i32>, req: HttpRequest) -> HttpResponse {
    let id = path.into_inner();
    let car = req
        .json::<CarDto>()
        .await
        .expect("failed to deserialize request body");
    let car_response = CarService::update_car(id, car).await;
    match car_response {
        Ok(response) => HttpResponse::build(StatusCode::OK).json(response),
        Err(error) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(error),
    }
}
