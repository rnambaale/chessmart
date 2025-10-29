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
}

pub use generated::matchmaker_service::matchmaker_service_server::{MatchmakerService, MatchmakerServiceServer};
pub use generated::matchmaker_service::{AddToQueueRequest, AddToQueueResponse, AcceptPendingGameRequest, AcceptPendingGameResponse, RemoveFromQueueRequest, RemoveFromQueueResponse, GetAccountStatusRequest, GetAccountStatusResponse, GetQueueSizesRequest, GetQueueSizesResponse};

pub type AddToQueueRequestPb = generated::matchmaker_service::AddToQueueRequest;

pub use generated::ranking_service::{GetAccountRankingRequest, GetAccountRankingResponse};
pub use generated::ranking_service::ranking_service_server::{RankingService, RankingServiceServer};

pub mod error;
pub mod primitives;
