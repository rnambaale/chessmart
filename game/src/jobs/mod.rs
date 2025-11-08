use serde::{Deserialize, Serialize};

use crate::jobs::check_game_job::CheckGamePayload;

pub mod check_game_job;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    CheckGameJob(CheckGamePayload),
    // MULTIPLY(MultiplyTask),
    // SUBTRACT(SubtractTask)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMessage {
    pub task: TaskType
}
