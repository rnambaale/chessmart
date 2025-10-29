use axum::{extract::State, Json};
use tracing::info;

use crate::{dtos::response::MeResponseDto, error::{AppResponseError, BunnyChessApiError}, server::state::AppState, utils::claim::UserClaims};

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
pub async fn find_account(
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
