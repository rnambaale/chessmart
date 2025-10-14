use axum::{
    routing::post,
    Router,
};

use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::authentication::{
    post_register, RegisterRequestDto, RegisterResponseDto,
};

pub async fn run_server() -> anyhow::Result<()> {

    info!("listening on: 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app().layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any)
                .expose_headers(Any),
        ).into_make_service()
    ).await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::authentication::post_register,
    ),
    components(schemas(
        RegisterRequestDto,
        RegisterResponseDto,
    ))
)]
struct ApiDoc;

fn app() -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // POST auth/login
        .route("/auth/register", post(post_register));

    router
}
