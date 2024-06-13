use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TicketEventType {
    Assigned,
    StatusUpdated,
    PriorityUpdated,
    TitleUpdated,
    DueDateUpdated,
}

//impl to string for storing in databaes (this_style)
impl ToString for TicketEventType {
    fn to_string(&self) -> String {
        match self {
            TicketEventType::Assigned => "assigned".to_string(),
            TicketEventType::StatusUpdated => "status_updated".to_string(),
            TicketEventType::PriorityUpdated => "priority_updated".to_string(),
            TicketEventType::TitleUpdated => "title_updated".to_string(),
            TicketEventType::DueDateUpdated => "due_date_updated".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TicketFilterPayload {
    pub assignee: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketCustomFieldDataPayload {
    pub ticket_id: i32,
    pub custom_field_id: i32,
    pub field_value: String,
}