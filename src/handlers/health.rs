use actix_web::HttpResponse;

pub async fn handler() -> HttpResponse {
    HttpResponse::Ok().json("ok")
}
