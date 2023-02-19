use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TicketEvent {
    pub event_id: Uuid,
    pub ticket_id: i32,
    pub event_type: String,
    pub event_data: String,
    pub user_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
}