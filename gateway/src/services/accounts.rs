use tracing::info;
use uuid::Uuid;

use crate::{database::{self, user::User, Database}, error::BunnyChessApiError, server::state::AppState};

pub async fn find_account(state: AppState, user_id: &Uuid) -> Result<User, BunnyChessApiError> {
    info!("Get user profile with id: {user_id}");
    let mut tx = state.db.begin_tx().await?;
    let user = database::user::get_by_id(&mut tx, user_id).await?;
    tx.commit().await?;
    Ok(user)
}
