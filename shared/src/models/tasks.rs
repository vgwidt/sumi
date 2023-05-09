use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskRepresentation {
    pub task_id: Uuid,
    pub tasklist_id: Uuid,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TasklistRepresentation {
    pub tasklist_id: Uuid,
    pub label: String,
    pub order_index: i32,
    pub tasks: Vec<TaskRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TicketTasklists {
    pub ticket_id: i32,
    pub tasklists: Vec<TasklistRepresentation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TaskNewPayload {
    pub tasklist_id: Uuid,
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
    pub tasklist_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_done: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TasklistUpdatePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
}