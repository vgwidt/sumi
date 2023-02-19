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

//Format event_type to prettify
impl ToString for TicketEvent {
    fn to_string(&self) -> String {
        match self.event_type.as_str() {
            "assigned" => if self.event_data == "unassigned" {
                format!("{} unassigned the ticket", self.user_id.unwrap())
            } else if self.event_data == "self" {
                format!("{} self-assigned the ticket", self.user_id.unwrap())
            } else {
                format!("{} assigned the ticket to {}", self.user_id.unwrap(), self.event_data)
            },
            "status_updated" => format!("{} set the status to {}", self.user_id.unwrap(), self.event_data),
            "priority_updated" => format!("{} set the priority to {}", self.user_id.unwrap(), self.event_data),
            "title_updated" => format!("{} changed the title to {}", self.user_id.unwrap(), self.event_data),
            _ => format!("Unknown event type: {}", self.event_type),
        }
    }
}
