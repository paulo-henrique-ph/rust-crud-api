// #[macro_use]
// extern crate diesel;
#[macro_use]
extern crate serde;

mod controllers;
mod configs;


use actix_web::{HttpServer, middleware::Logger, App};
use configs::{
    cors::with_cors,
    logger::{end_telemetry, init_telemetry, with_logger},
    open_api::with_swagger
};
use controllers::routes::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    init_telemetry();

    HttpServer::new(|| {
        let app = App::new()
            .wrap(Logger::default())
            .wrap(with_cors())
            .wrap(with_logger())
            .configure(configure_routes);

        match is_dev() {
            true => app.service(with_swagger()),
            false => app,
        }
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    end_telemetry();

    Ok(())
}

fn is_dev() -> bool {
    true
}
