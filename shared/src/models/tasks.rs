use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskRepresentation {
    pub task_id: Uuid,
    pub group_id: Uuid,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskGroupRepresentation {
    pub group_id: Uuid,
    pub label: String,
    pub order_index: i32,
    pub tasks: Vec<TaskRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tasklist {
    pub ticket_id: i32,
    pub task_groups: Vec<TaskGroupRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskNewPayload {
    pub group_id: Uuid,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupNewPayload {
    pub label: String,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskUpdatePayload {
    pub group_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskGroupUpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
}