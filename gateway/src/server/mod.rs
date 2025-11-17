use std::net::SocketAddr;

use axum::{
    routing::{get, post}, Router
};

use socketioxide::{SocketIo, handler::ConnectHandler};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub mod state;

use crate::{ dtos::{request::{LoginRequestDto, RefreshTokenRequestDto, RegisterRequestDto}, response::{AccountResponseDto, LoginResponseDto, MeResponseDto, MessageResponseDto, RegisterResponseDto}}, routes::{accounts::{get_account, me}, authentication::{login, logout, post_register, refresh}, websocket::ws_handler}, server::state::AppState, utils::claim::UserClaims};

pub async fn run_server(state: AppState) -> anyhow::Result<()> {

    info!("listening on: 3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    let (layer, io) = SocketIo::new_layer();

    // let (layer, io) = SocketIo::builder()
    //     .with_state(state.clone())
    //     .build_layer();

    io.ns("/", crate::handlers::on_connect.with(crate::handlers::authenticate_middleware));

    // io.ns("/socket.io", |s: SocketRef| {
    //     s.on("matchmaking:add-to-queue", crate::routes::matchmaking::handle_add_to_queue);
    //     s.on("matchmaking:remove-from-queue", crate::routes::matchmaking::handle_remove_from_queue);
    //     s.on("matchmaking:accept-pending-game", crate::routes::matchmaking::handle_accept_pending_game);
    //     s.on("matchmaking:join-game", crate::routes::matchmaking::handle_join_game);
    //     s.on("matchmaking:join-lobby", crate::routes::matchmaking::handle_join_lobby);
    //     s.on("matchmaking:leave-lobby", crate::routes::matchmaking::handle_leave_lobby);
    // });

    axum::serve(
        listener,
        app(state)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers(Any)
                    .allow_methods(Any)
                    .expose_headers(Any),
            )
            .layer(layer)
        //.into_make_service()
        .into_make_service_with_connect_info::<SocketAddr>()
    ).await?;

    Ok(())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::authentication::post_register,
        crate::routes::authentication::login,
        crate::routes::authentication::refresh,
        crate::routes::authentication::logout,
        crate::routes::accounts::get_account,
        crate::routes::accounts::me,
    ),
    components(schemas(
        RegisterRequestDto,
        RegisterResponseDto,
        LoginRequestDto,
        LoginResponseDto,
        RefreshTokenRequestDto,
        UserClaims,
        MessageResponseDto,
        AccountResponseDto,
        MeResponseDto,
    ))
)]
struct ApiDoc;

fn app(state: AppState) -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))

        // Auth routes
        .route("/auth/register", post(post_register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh))
        .route("/auth/logout", post(logout))

        // WebSocket route
        .route("/ws", get(ws_handler))

        // Account routes
        .route("/accounts/me", get(me))
        .route("/accounts/:account_id", get(get_account))

        .with_state(state);

    router
}
