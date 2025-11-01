use shared::{AccountServiceServer, FindAccountRequest, LoginRequest, RefreshRequest, RegisterRequest};
use tonic::transport::Server;
use prost_types::Timestamp;

use crate::{config::ApiConfig, database::user::Account, dtos::response::LoginResponseDto, state::state::{AppState, AppStateBuilder}, utils::timestamps::TimestampExt};

pub mod services;
pub mod database;
pub mod redis;
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
        request: tonic::Request<RegisterRequest>,
    ) -> Result<tonic::Response<shared::Account>, tonic::Status> {
        let Account {
            id,
            email,
            username,
            is_admin,
            created_at,
            last_login_at,
            ..
        } = crate::services::account_service::register(&self.state, request.into_inner()).await?;

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
        request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<shared::LoginResponse>, tonic::Status> {
        let LoginResponseDto {
            jwt_expires_in,
            jwt_refresh,
            jwt,
            jwt_refresh_expires_in,
        } = crate::services::account_service::login(&self.state, request.into_inner()).await?;

        Ok(tonic::Response::new(shared::LoginResponse{
            jwt,
            jwt_expires: Some(Timestamp::from_chrono(jwt_expires_in)),
            jwt_refresh,
            jwt_refresh_expires: Some(Timestamp::from_chrono(jwt_refresh_expires_in))
        }))
    }

    async fn refresh(
        &self,
        _request: tonic::Request<RefreshRequest>,
    ) -> std::result::Result<tonic::Response<shared::LoginResponse>, tonic::Status> {
        todo!()
    }

    async fn find_account(
        &self,
        _request: tonic::Request<FindAccountRequest>,
    ) -> std::result::Result<tonic::Response<shared::Account>, tonic::Status> {
        todo!()
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    println!("AuthenticationService gRPC server running on {}", addr);

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

    let account_service = AccountGatewayService::new(
        state
    );

    Server::builder()
        .add_service(AccountServiceServer::new(account_service))
        .serve(addr).await?;

    Ok(())
}
