use shared::{AccountServiceServer, error::BunnyChessApiError};
use tonic::transport::Server;
use prost_types::Timestamp;
use tracing::{info, warn};

use crate::{config::ApiConfig, repositories::user::Account, dtos::{request::{FindAccountRequestDto, LoginRequestDto, RefreshTokenRequestDto, RegisterRequestDto}, response::LoginResponseDto}, state::state::{AppState, AppStateBuilder}, utils::timestamps::TimestampExt};

pub mod services;
pub mod repositories;
pub mod client;
mod config;
mod state;
pub mod utils;
pub mod constants;
pub mod dtos;

pub struct AccountGatewayService {
    state: AppState
}

impl AccountGatewayService {
    pub fn new(
        state: AppState
    ) -> Self {
        Self { state }
    }
}

#[tonic::async_trait]
impl shared::AccountService for AccountGatewayService {
    async fn register(
        &self,
        request: tonic::Request<shared::RegisterRequest>,
    ) -> Result<tonic::Response<shared::Account>, tonic::Status> {
        let shared::RegisterRequest {
            email,
            username,
            password,
            is_admin: _,
        } = request.into_inner();

        let request = RegisterRequestDto { email, username, password };

        let Account {
            id,
            email,
            username,
            is_admin,
            created_at,
            last_login_at,
            ..
        } = crate::services::account_service::register(&self.state, &request).await?;

        let last_login_at = match last_login_at {
            Some(login_at) => Some(Timestamp::from_chrono(login_at)),
            None => None,
        };

        Ok(tonic::Response::new(shared::Account{
            id: id.to_string(),
            email,
            username,
            is_admin,
            created_at: Some(Timestamp::from_chrono(created_at)),
            last_login_at
        }))
    }

    async fn login(
        &self,
        request: tonic::Request<shared::LoginRequest>,
    ) -> std::result::Result<tonic::Response<shared::LoginResponse>, tonic::Status> {
        let shared::LoginRequest {
            email,
            password
        } = request.into_inner();

        let request = LoginRequestDto { email, password };

        let LoginResponseDto {
            jwt_expires_in,
            jwt_refresh,
            jwt,
            jwt_refresh_expires_in,
        } = crate::services::account_service::login(&self.state, &request).await?;

        Ok(tonic::Response::new(shared::LoginResponse {
            jwt,
            jwt_expires: Some(Timestamp::from_chrono(jwt_expires_in)),
            jwt_refresh,
            jwt_refresh_expires: Some(Timestamp::from_chrono(jwt_refresh_expires_in))
        }))
    }

    async fn refresh(
        &self,
        request: tonic::Request<shared::RefreshRequest>,
    ) -> std::result::Result<tonic::Response<shared::LoginResponse>, tonic::Status> {
        let shared::RefreshRequest { jwt_refresh } = request.into_inner();

        let request = RefreshTokenRequestDto { token: jwt_refresh };

        match crate::services::token::refresh(&self.state, &request).await {
            Ok(response) => {
                info!("Success refresh token user response: {response:?}.");
                let LoginResponseDto {
                    jwt_expires_in,
                    jwt_refresh,
                    jwt,
                    jwt_refresh_expires_in,
                } = response;

                Ok(tonic::Response::new(shared::LoginResponse {
                    jwt,
                    jwt_expires: Some(Timestamp::from_chrono(jwt_expires_in)),
                    jwt_refresh,
                    jwt_refresh_expires: Some(Timestamp::from_chrono(jwt_refresh_expires_in))
                }))
            }
            Err(e) => {
                warn!("Unsuccessfully refresh token error: {e:?}.");
                return Err(e.into());
            }
        }
    }

    async fn find_account(
        &self,
        request: tonic::Request<shared::FindAccountRequest>,
    ) -> std::result::Result<tonic::Response<shared::Account>, tonic::Status> {
        let shared::FindAccountRequest { id, email } = request.into_inner();

        let keys = vec![id.clone(), email.clone()];

        if keys.into_iter().filter(|key| key.is_some()).count() != 1 {
            return Err(
                BunnyChessApiError::InvalidInputError("Exactly one between 'id' and 'email' must be set".to_string()).into()
            );
        }

        let request = FindAccountRequestDto { id, email };

        let Account {
            id,
            email,
            username,
            is_admin,
            created_at,
            last_login_at,
            ..
        } = crate::services::account_service::find_account(&self.state, &request).await?;

        let last_login_at = match last_login_at {
            Some(login_at) => Some(Timestamp::from_chrono(login_at)),
            None => None,
        };

        Ok(tonic::Response::new(shared::Account {
            id: id.to_string(),
            email,
            username,
            is_admin,
            created_at: Some(Timestamp::from_chrono(created_at)),
            last_login_at
        }))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ApiConfig {
        server,
        database,
        tracing,
        redis,
        token_secret,
    } = ApiConfig::read_config_with_defaults();

    let state: AppState = AppStateBuilder::new()
        .with_server(Some(server))
        .with_db(Some(database))
        .with_tracing(tracing)
        .with_redis(Some(redis))
        .with_token_secret(Some(token_secret))
        .build()
        .await?;

    // let addr = "[::1]:50051".parse()?;
    let addr = state.config.server.host_port;

    println!("AuthenticationService gRPC server running on {}", addr);

    let account_service = AccountGatewayService::new(
        state
    );

    Server::builder()
        .add_service(AccountServiceServer::new(account_service))
        .serve(addr).await?;

    Ok(())
}
