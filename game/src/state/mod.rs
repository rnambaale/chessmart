use shared::{GameServiceServer, error::ChessmartApiError};
use tonic::transport::Server;

use crate::{GameGatewayService, state::state::AppState};

pub mod state;
pub mod worker;

pub struct AppServer {
    pub state: AppState
}

impl AppServer {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn run(self) -> Result<(), ChessmartApiError> {
        let addr = self.state.config.server.host_port;

        println!("GameService gRPC server running on {}", addr);

        let game_service = GameGatewayService::new(
            self.state.clone()
        );

        Server::builder()
            .add_service(GameServiceServer::new(game_service))
            .serve(addr)
            .await?;

        Ok(())
    }
}
