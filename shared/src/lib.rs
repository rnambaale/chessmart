// Include the generated gRPC code
pub mod generated {
    #![allow(clippy::all)]
    // include!("generated/user_service.rs");
    // tonic::include_proto!("user_service");
    pub mod matchmaker_service {
        include!("generated/matchmaker.rs");
    }

    pub mod ranking_service {
        include!("generated/ranking.rs");
    }

    pub mod account_service {
        include!("generated/account.rs");
    }

    pub mod game_service {
        include!("generated/game.rs");
    }
}

pub use generated::matchmaker_service::matchmaker_service_server::{MatchmakerService, MatchmakerServiceServer};
pub use generated::matchmaker_service::{AddToQueueRequest, AddToQueueResponse, AcceptPendingGameRequest, AcceptPendingGameResponse, RemoveFromQueueRequest, RemoveFromQueueResponse, GetAccountStatusRequest, GetAccountStatusResponse, GetQueueSizesRequest, GetQueueSizesResponse};

pub type AddToQueueRequestPb = generated::matchmaker_service::AddToQueueRequest;
pub use generated::matchmaker_service::QueueSize;

pub use generated::ranking_service::{GetAccountRankingRequest, GetAccountRankingResponse};
pub use generated::ranking_service::ranking_service_server::{RankingService, RankingServiceServer};

pub use generated::account_service::{RegisterRequest, Account, LoginRequest, LoginResponse, RefreshRequest, FindAccountRequest};
pub use generated::account_service::account_service_server::{AccountService, AccountServiceServer};

pub use generated::game_service::{CreateGameRequest, CreateGameResponse, MakeMoveRequest, MakeMoveResponse, GetGameStateRequest, GetGameStateResponse, CheckGameResultRequest, CheckGameResultResponse, ResignRequest, ResignResponse};
pub use generated::game_service::game_service_server::{GameService, GameServiceServer};

pub mod error;
pub mod primitives;
pub mod events;
