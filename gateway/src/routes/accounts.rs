use axum::{extract::State, Json};
use prost_types::Timestamp;
use shared::{Account, GetAccountRankingResponse, primitives::TimestampExt};
use tracing::info;

use crate::{dtos::response::{AccountResponseDto, MeResponseDto}, error::{AppResponseError, BunnyChessApiError}, server::state::AppState, utils::claim::UserClaims};

#[utoipa::path(
    get,
    path = "/accounts/me",
    responses(
        (status = 200, description = "Success get user profile", body = [MeResponseDto]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn me(
  State(_state): State<AppState>,
  user: UserClaims,
) -> Result<Json<MeResponseDto>, BunnyChessApiError> {
    info!("Get profile user id: {}.", user.uid);

    todo!()
//   match services::accounts::get_profile(&state, user.uid).await {
//     Ok(resp) => {
//       info!("Success get profile user: {}.", user.uid);
//       Ok(Json(resp))
//     }
//     Err(e) => {
//       warn!("Unsuccessfully get profile user: {e:?}.");
//       Err(e)
//     }
//   }
}

#[utoipa::path(
    get,
    path = "/accounts/:account_id",
    responses(
        (status = 200, description = "Success get user profile", body = [AccountResponseDto]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_account(
    State(state): State<AppState>,
    axum::extract::Path(account_id): axum::extract::Path<String>,
    _user: UserClaims,
) -> Result<Json<AccountResponseDto>, BunnyChessApiError> {
    info!("Get profile user id: {}.", account_id);

    let Account {
        username,
        is_admin,
        created_at,
        last_login_at,
        ..
    } = state
        .account_client.clone()
        .find_account(
            shared::FindAccountRequest { id: Some(account_id.to_string()), email: None }
        ).await?
        .into_inner();

    let GetAccountRankingResponse {
        ranked_mmr, ..
    } = state
        .ranking_client.clone()
        .get_account_ranking(
            shared::GetAccountRankingRequest { account_id: account_id.to_string() }
        ).await?
        .into_inner();

    let created_at = match created_at {
        Some(created_at) => Some(Timestamp::to_chrono(&created_at)),
        None => None,
    };

    let last_login_at = match last_login_at {
        Some(last_login_at) => Some(Timestamp::to_chrono(&last_login_at)),
        None => None,
    };

    let response = AccountResponseDto {
        id: account_id.to_string(),
        username,
        is_admin,
        created_at,
        last_login_at,
        mmr: ranked_mmr as u64,
    };

    Ok(Json(response))
}
