use std::str::FromStr;

use futures::FutureExt;

use crate::{config::ApiConfig, primitives::CreateGameDto, state::{AppServer, state::{AppState, AppStateBuilder}, worker::Worker}};

mod client;
mod config;
mod error;
mod jobs;
mod primitives;
mod repositories;
mod services;
mod state;
mod utils;
pub struct GameGatewayService {
    state: AppState,
    // game_service: GameService,
}

impl GameGatewayService {
    pub fn new(
        state: AppState
    ) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl shared::GameService for GameGatewayService {
    async fn create_game(
        &self,
        request: tonic::Request<shared::CreateGameRequest>,
    ) -> Result<tonic::Response<shared::CreateGameResponse>, tonic::Status> {
        let shared::CreateGameRequest {
            account_id0,
            account_id1,
            game_type,
            metadata,
        } = request.into_inner();

        let game_type = shared::primitives::GameType::from_str(&game_type)?;

        let chess_game = crate::services::game_service::create_game(
            &self.state,
            CreateGameDto {
                account_id0,
                account_id1,
                game_type,
                metadata
            }
        ).await?;

        Ok(tonic::Response::new(shared::CreateGameResponse{
            game_id: chess_game.id.clone(),
            game_repr: chess_game.to_string()
        }))
    }

    async fn get_game_state(
        &self,
        request: tonic::Request<shared::GetGameStateRequest>,
    ) -> Result<tonic::Response<shared::GetGameStateResponse>, tonic::Status> {
        let shared::GetGameStateRequest { game_id } = request.into_inner();

        let chess_game = crate::services::game_service::get_game(
            &self.state,
            game_id.as_str()
        ).await?;

        Ok(tonic::Response::new(shared::GetGameStateResponse {
            game_repr: chess_game.to_string()
        }))
    }

    async fn check_game_result(
        &self,
        request: tonic::Request<shared::CheckGameResultRequest>,
    ) -> Result<tonic::Response<shared::CheckGameResultResponse>, tonic::Status> {
        let shared::CheckGameResultRequest { game_id } = request.into_inner();

        let chess_game = crate::services::game_service::get_game(
            &self.state,
            game_id.as_str()
        ).await?;

        crate::services::game_service::check_game_result(
            &self.state,
            &chess_game
        ).await?;

        Ok(tonic::Response::new(shared::CheckGameResultResponse {}))
    }

    async fn make_move(
        &self,
        request: tonic::Request<shared::MakeMoveRequest>,
    ) -> Result<tonic::Response<shared::MakeMoveResponse>, tonic::Status> {
        let shared::MakeMoveRequest { game_id, r#move,  account_id } = request.into_inner();

        let chess_game = crate::services::game_service::make_move(
            &self.state,
            &game_id,
            &account_id,
            &r#move,
        ).await?;

        Ok(tonic::Response::new(shared::MakeMoveResponse {
            game_repr: chess_game.to_string(),
        }))
    }

    async fn resign(
        &self,
        request: tonic::Request<shared::ResignRequest>,
    ) -> Result<tonic::Response<shared::ResignResponse>, tonic::Status> {
        let shared::ResignRequest { game_id, account_id } = request.into_inner();

        let chess_game = crate::services::game_service::resign(
            &self.state,
            &game_id,
            &account_id,
        ).await?;

        Ok(tonic::Response::new(shared::ResignResponse {
            game_repr: chess_game.to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    dotenvy::from_filename(format!("{}/.env", crate_dir)).ok();

    let ApiConfig {
        server,
        tracing,
        redis,
        nats
    } = ApiConfig::read_config_with_defaults();

    let state: AppState = AppStateBuilder::new()
        .with_server(Some(server))
        .with_tracing(tracing)
        .with_redis(Some(redis))
        .with_nats(Some(nats))
        .build()
        .await?;

    let worker = Worker::new(state.clone());
    let server = AppServer::new(state.clone());

    utils::task::join_all(vec![
        (true, server.run().boxed()),
        (true, worker.run().boxed()),
    ])
    .await?;

    Ok(())
}
