pub struct PlayerStatusRepositoryService {}

impl PlayerStatusRepositoryService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_account_status_key(account_id: &str) -> String {
        format!("matchmaking:account:{}:status", account_id)
    }
}
