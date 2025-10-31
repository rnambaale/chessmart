use shared::{AccountServiceServer, FindAccountRequest, LoginRequest, LoginResponse, RefreshRequest, RegisterRequest};
use tonic::transport::Server;

use crate::{config::ApiConfig, repositories::account_repository::{Account, AccountRepository}, server::state::{AppState, AppStateBuilder}, services::account_service::AccountService, utils::timestamps::TimestampExt};
use prost_types::Timestamp;

pub mod services;
pub mod repositories;
pub mod database;
pub mod redis;
mod config;
mod server;
pub mod utils;

pub struct AccountGatewayService {
    account_service: AccountService
}

impl AccountGatewayService {
    pub fn new(
        account_service: AccountService
    ) -> Self {
        Self { account_service }
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
        } = self.account_service.register(request.into_inner()).await?;

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
        _request: tonic::Request<LoginRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
        todo!()
    }

    async fn refresh(
        &self,
        _request: tonic::Request<RefreshRequest>,
    ) -> std::result::Result<tonic::Response<LoginResponse>, tonic::Status> {
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
        AccountService::new(AccountRepository::new(state.db.clone()))
    );

    Server::builder()
        .add_service(AccountServiceServer::new(account_service))
        .serve(addr).await?;

    Ok(())
}
