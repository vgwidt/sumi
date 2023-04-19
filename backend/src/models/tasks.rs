use crate::schema::tasks;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Task {
    pub task_id: Uuid,
    pub ticket_id: i32,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub task_id: Uuid,
    pub ticket_id: i32,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = tasks)]
pub struct TaskPayload {
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}
