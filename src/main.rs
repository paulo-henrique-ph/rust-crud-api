// #[macro_use]
// extern crate diesel;
#[macro_use]
extern crate serde;

use actix_web::web::Data;
use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing::info;

use configs::{cors::with_cors, open_api::with_swagger};

use crate::application_context::ApplicationContext;
use crate::configs::environment::Env;
use crate::configs::logger;
use crate::handlers::{car, health};

mod application_context;
mod configs;
mod error;
mod handlers;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env = Env::load();

    let context = ApplicationContext::autowire(&env)
        .await
        .expect("failed to create application context");

    let _guard = logger::setup();

    let span = tracing::info_span!(
        "http.request",
        otel.kind = "server",
        http.route = "/test",
        http.method = "POST",
    );

    span.in_scope(|| {
        info!("handling request");
    });
    HttpServer::new(move || {
        let app = App::new()
            .wrap(Logger::default())
            .wrap(with_cors())
            .route("/health", web::get().to(health::handler))
            .service(
                web::scope("/car")
                    .route("/create", web::post().to(car::create_handler))
                    .route("/delete", web::delete().to(car::delete_handler)),
            )
            .app_data(Data::new(context.clone()));

        match is_dev() {
            true => app.service(with_swagger()),
            false => app,
        }
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

fn is_dev() -> bool {
    true
}
