use crate::schema::{tasks, task_groups, task_templates, task_template_groups, task_template_tasks};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TaskGroup {
    pub group_id: Uuid,
    pub ticket_id: i32,
    pub label: String,
    pub order_index: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = task_groups)]
pub struct NewTaskGroup {
    pub ticket_id: i32,
    pub label: String,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = task_groups)]
pub struct TaskGroupPayload {
    pub label: String,
    pub order_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Task {
    pub task_id: Uuid,
    pub group_id: Uuid,
    pub label: String,
    pub is_done: bool,
    pub order_index: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub group_id: Uuid,
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

