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

#[tracing::instrument]
pub fn with_swagger() -> SwaggerUi {
    // println!("Visit Swagger UI at http://{}:{}/swagger-ui/#", HOSTNAME, PORT);
    tracing::error!("oops");

    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi())
}
