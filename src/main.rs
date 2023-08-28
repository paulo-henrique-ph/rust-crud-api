mod controllers;

use actix_cors::Cors;
use actix_web::{HttpServer, middleware::Logger, http::header, App};
use controllers::health_controller::health_checker_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(health_checker_handler)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
