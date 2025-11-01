use chrono::Utc;
use shared::{LoginRequest, RegisterRequest, error::BunnyChessApiError};
use uuid::Uuid;

use crate::{database::{Database, user::Account}, dtos::response::LoginResponseDto, state::state::AppState};

pub async fn register(
    state: &AppState,
    req: RegisterRequest
) -> Result<Account, BunnyChessApiError> {
    let mut tx = state.db.begin_tx().await?;

    let account = Account {
        id: Uuid::new_v4(),
        email: req.email.to_string(),
        username: req.username.to_string(),
        password: crate::utils::password::hash(req.password.to_string()).await?,
        is_admin: false,
        last_login_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    crate::database::user::insert_account(&mut tx, &account).await?;

    tx.commit().await?;

    Ok(account)
}

pub async fn login(
    state: &AppState,
    req: LoginRequest
) -> Result<LoginResponseDto, BunnyChessApiError> {
    let mut tx = state.db.begin_tx().await?;
    let account = crate::database::user::find_account_by_email(&mut tx, &req.email).await?;

    if account.is_none() {
        return Err(BunnyChessApiError::EmailNotFoundError("Email not found.".into()));
    }

    let account = account.unwrap();

    crate::utils::password::verify(req.password.clone(), account.password.clone()).await?;

    crate::database::user::update_last_login(&mut tx, &account).await?;

    tx.commit().await?;

    let session_id = crate::services::session::set(&state.redis, account.id).await?;
    let resp = crate::services::token::generate_tokens(account.id, session_id)?;

    Ok(resp)
}
