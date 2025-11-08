use std::thread;

use serde::{Deserialize, Serialize};

pub const CHECK_GAME_QUEUE_NAME: &str = "check_game_queue";
pub const CHECK_GAME_JOB_NAME: &str = "check_game";


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckGamePayload {
    pub game_id: String,
}

impl CheckGamePayload {
    pub fn run(self) -> () {
        // do actual work

        let duration = std::time::Duration::from_secs(20);
        thread::sleep(duration);
    }
}
