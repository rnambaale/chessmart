use shared::error::BunnyChessApiError;
use tracing::info;

use crate::{jobs::TaskMessage, state::state::AppState};
use crate::jobs::TaskType::CheckGameJob;

pub struct Worker {
    state: AppState,
}

impl Worker {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn run(self) -> Result<(), BunnyChessApiError> {
        println!("The game task worker started.");

        loop {
            let outcome: Option<Vec<u8>> = {
                let mut conn = self.state.redis.get_connection().unwrap();

                redis::cmd("RPOP")
                    .arg(crate::jobs::check_game_job::CHECK_GAME_JOB_NAME)
                    .query(&mut conn)
                    .unwrap()
            };

            match outcome {
                Some(data) => {
                    let deserialized_message: TaskMessage = bincode::deserialize(&data).unwrap();
                    match deserialized_message.task {
                        CheckGameJob(task) => {
                            info!("Running Check game task");
                            task.run();
                        },
                    }
                    todo!()
                }
                None => {
                    info!("check_game_queue empty");
                    let five_seconds = std::time::Duration::from_secs(5);
                    tokio::time::sleep(five_seconds).await;
                }
            }

        }
    }
}
