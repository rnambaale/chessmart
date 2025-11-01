use chrono::Utc;
use shared::{LoginRequest, RegisterRequest, error::BunnyChessApiError};
use uuid::Uuid;

use crate::{database::{Database, postgres::PostgresDB, user::Account}, dtos::response::LoginResponseDto, state::state::AppState};

pub async fn register(
    state: &AppState,
    req: RegisterRequest
) -> Result<Account, BunnyChessApiError> {
    let mut tx = state.db.begin_tx().await?;

    check_unique_username(&mut tx, &req.username).await?;
    check_unique_email(&mut tx, &req.email).await?;

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

async fn check_unique_username(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
    username: &str,
) -> Result<bool, BunnyChessApiError> {
    let username_option = crate::database::user::find_account_by_username(tx, username).await?;

    let username_exists = match username_option {
        Some(_) => true,
        None => false,
    };

    if username_exists {
        return Err(BunnyChessApiError::UserAlreadyExists);
    }

    Ok(true)
}

pub async fn check_unique_email(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
    email: &str,
) -> Result<bool, BunnyChessApiError> {
    let email_result = crate::database::user::find_account_by_email(tx, email).await;

    let email_exists = match email_result {
        Ok(_) => true,
        Err(BunnyChessApiError::Db(sqlx::Error::RowNotFound)) => false,
        Err(e) => return Err(e),
    };

    if email_exists {
        return Err(BunnyChessApiError::UserAlreadyExists);
    }

    Ok(true)
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
    let response = crate::services::token::generate_tokens(account.id, session_id)?;
    Ok(response)
}
