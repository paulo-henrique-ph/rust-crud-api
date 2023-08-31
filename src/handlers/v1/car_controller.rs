use actix_web::{delete, get, post, put, web, Responder};
#[utoipa::path(
path = "/api/v1/cars",
tags = "Cars"
)]
#[get("")]
pub async fn get_all_cars() -> impl Responder {
    "Get all cars"
}

#[utoipa::path(
path = "/api/v1/cars/{id}",
tags = "Cars"
)]
#[get("/{id}")]
pub async fn get_car(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();
    "Get all cars"
}

#[utoipa::path(
path = "/api/v1/cars/",
request_body = UpdateTodoRequest,
responses(
(status = 200, description = "Created a todo item successfully", body = Car)
),
tag = "Cars"
)]
#[post("")]
pub async fn create_car(
    db_pool: web::Data<PostgresPool>,
    req_body: web::Json<Car>,
) -> impl Responder {
    let req_body = &req_body.into_inner();

    db_pool.run(|con| {
        diesel::insert_into(todos::table)
            .values(req_body)
            .get_result::<Todo>(con)
            .into_res("Error creating todo")
    })
}