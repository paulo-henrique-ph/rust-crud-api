use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// use constants::{HOSTNAME, PORT};
#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "health", description = "Health related endpoints."),
    ),
)]
struct ApiDoc;

pub fn with_swagger() -> SwaggerUi {
    // println!("Visit Swagger UI at http://{}:{}/swagger-ui/#", HOSTNAME, PORT);
    println!("Visit Swagger UI at http://localhost:8080/swagger-ui/#");

    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}