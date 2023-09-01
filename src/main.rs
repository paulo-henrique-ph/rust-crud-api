#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use actix_web::web::Data;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::fmt::Debug;

use configs::{cors::with_cors, open_api::with_swagger};

use crate::application_context::ApplicationContext;
use crate::configs::environment::Env;
use crate::configs::telemetry;
use crate::handlers::{health, v1::car};

mod application_context;
mod configs;
mod dto;
mod entities;
mod handlers;
mod models;
mod repositories;
mod services;
mod utils;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env = Env::load();

    let context = ApplicationContext::autowire(&env)
        .await
        .expect("failed to create application context");

    let _guard = telemetry::init_tracing_subscriber();

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default())
            .wrap(with_cors())
            .route("/health", web::get().to(health::handler))
            .service(
                web::scope("api/v1/car")
                    .route("/create", web::post().to(car::create_handler))
                    .route("/delete", web::delete().to(car::delete_handler))
                    .route("/{id}", web::get().to(car::get_handler))
                    .route("/update", web::put().to(car::update_handler)),
            )
            .app_data(Data::new(context.clone()));

        match env.is_dev {
            true => app.service(with_swagger()),
            false => app,
        }
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
