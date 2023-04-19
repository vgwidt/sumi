use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskRepresentation {
    pub task_id: Uuid,
    pub ticket_id: i32,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tasklist {
    pub ticket_id: i32,
    pub tasks: Vec<TaskRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskNewPayload {
    pub ticket_id: i32,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdatePayload {
    pub ticket_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
}