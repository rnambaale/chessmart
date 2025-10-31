use chrono::Utc;
use shared::{RegisterRequest, error::BunnyChessApiError};
use uuid::Uuid;

use crate::repositories::account_repository::{Account, AccountRepository};

pub struct AccountService {
    account_repository: AccountRepository,
}

impl AccountService {
    pub fn new(
        account_repository: AccountRepository,
    ) -> Self {
        Self { account_repository }
    }

    pub async fn register(
        &self,
        req: RegisterRequest
    ) -> Result<Account, BunnyChessApiError> {
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

        self.account_repository.insert_account(&account).await?;

        Ok(account)
    }
}


