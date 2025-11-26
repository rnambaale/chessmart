use std::str::FromStr;

use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::{client::database::{Database, PostgresDB}, dtos::{request::{FindAccountRequestDto, LoginRequestDto, RegisterRequestDto}, response::LoginResponseDto}, error::AuthServiceError, repositories::user::Account, services::redis::SessionKey, state::state::AppState};

pub async fn register(
    state: &AppState,
    request: &RegisterRequestDto
) -> Result<Account, AuthServiceError> {
    let mut tx = state.db.begin_tx().await?;

    check_unique_username(&mut tx, &request.username).await?;
    check_unique_email(&mut tx, &request.email).await?;

    let account = Account {
        id: Uuid::new_v4(),
        email: request.email.to_string(),
        username: request.username.to_string(),
        password: crate::utils::password::hash(request.password.to_string()).await?,
        is_admin: false,
        last_login_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    crate::repositories::user::insert_account(&mut tx, &account).await?;

    tx.commit().await?;

    Ok(account)
}

async fn check_unique_username(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
    username: &str,
) -> Result<bool, AuthServiceError> {
    let username_option = crate::repositories::user::find_account_by_username(tx, username).await?;

    let username_exists = match username_option {
        Some(_) => true,
        None => false,
    };

    if username_exists {
        return Err(AuthServiceError::UserAlreadyExists);
    }

    Ok(true)
}

pub async fn check_unique_email(
    tx: &mut sqlx::Transaction<'_, <PostgresDB as Database>::DB>,
    email: &str,
) -> Result<bool, AuthServiceError> {
    let email_result = crate::repositories::user::find_account_by_email(tx, email).await;

    let email_exists = match email_result {
        Ok(_) => true,
        Err(AuthServiceError::Db(sqlx::Error::RowNotFound)) => false,
        Err(e) => return Err(e),
    };

    if email_exists {
        return Err(AuthServiceError::UserAlreadyExists);
    }

    Ok(true)
}

pub async fn login(
    state: &AppState,
    request: &LoginRequestDto
) -> Result<LoginResponseDto, AuthServiceError> {
    let mut tx = state.db.begin_tx().await?;
    let account = crate::repositories::user::find_account_by_email(&mut tx, &request.email).await?;

    if account.is_none() {
        return Err(AuthServiceError::EmailNotFoundError("Email not found.".into()));
    }

    let account = account.unwrap();

    crate::utils::password::verify(request.password.clone(), account.password.clone()).await?;

    crate::repositories::user::update_last_login(&mut tx, &account).await?;

    tx.commit().await?;

    let session_id = crate::services::session::set(&state.redis, account.id).await?;
    let response = crate::services::token::generate_tokens(account.id, session_id)?;
    Ok(response)
}

pub async fn find_account(
    state: &AppState,
    request: &FindAccountRequestDto
) -> Result<Account, AuthServiceError> {
    let FindAccountRequestDto { id, email} = request;

    if let Some(id) = id {
        let id = Uuid::from_str(id)?;

        let mut tx = state.db.begin_tx().await?;
        let account = crate::repositories::user::get_by_id(&mut tx, &id).await?;
        tx.commit().await?;

        return Ok(account);
    }

    if let Some(email) = email {
        let mut tx = state.db.begin_tx().await?;
        let account_option = crate::repositories::user::find_account_by_email(&mut tx, &email).await?;
        tx.commit().await?;

        let account = match account_option {
            Some(account ) => account,
            None => {
                return Err(AuthServiceError::EmailNotFoundError("User with email not found".into()));
            }
        };

        return Ok(account);
    }

    Err(
        AuthServiceError::InvalidInputError("Either 'id' and 'email' must be set".to_string())
    )
}

pub async fn logout(state: &AppState, user_id: Uuid) -> Result<(), AuthServiceError> {
    info!("Logout user id: {user_id}");
    let key = SessionKey { user_id };
    crate::services::redis::del(&state.redis, &key).await?;
    Ok(())
}
