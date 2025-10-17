use axum::{
    routing::post,
    Router,
};

use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod state;

use crate::{ dtos::{request::{LoginRequestDto, RefreshTokenRequestDto, RegisterRequestDto}, response::{LoginResponseDto, RegisterResponseDto}}, routes::authentication::{login, post_register, refresh}, server::state::AppState};

pub async fn run_server(state: AppState) -> anyhow::Result<()> {

    info!("listening on: 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app(state).layer(
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
        crate::routes::authentication::login,
        crate::routes::authentication::refresh,
    ),
    components(schemas(
        RegisterRequestDto,
        RegisterResponseDto,
        LoginRequestDto,
        LoginResponseDto,
        RefreshTokenRequestDto,
    ))
)]
struct ApiDoc;

fn app(state: AppState) -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/auth/register", post(post_register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .with_state(state);

    router
}
