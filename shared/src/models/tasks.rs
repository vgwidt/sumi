use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskRepresentation {
    pub task_id: Uuid,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupRepresentation {
    pub group_id: Uuid,
    pub label: String,
    pub order_index: i32,
    pub tasks: Vec<TaskRepresentation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tasklist {
    pub ticket_id: i32,
    pub task_groups: Vec<TaskGroupRepresentation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdatePayload {
    pub label: Option<String>,
    pub is_done: Option<bool>,
    pub order_index: Option<i32>,
}