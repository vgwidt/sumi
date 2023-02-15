use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TicketEventType {
    Archived,
    Unarchived,
    Assigned,
    StatusUpdated,
    PriorityUpdated,
    TitleUpdated,
}

//impl to string for storing in databaes (this_style)
impl ToString for TicketEventType {
    fn to_string(&self) -> String {
        match self {
            TicketEventType::Archived => "archived".to_string(),
            TicketEventType::Unarchived => "unarchived".to_string(),
            TicketEventType::Assigned => "assigned".to_string(),
            TicketEventType::StatusUpdated => "status_updated".to_string(),
            TicketEventType::PriorityUpdated => "priority_updated".to_string(),
            TicketEventType::TitleUpdated => "title_updated".to_string(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TicketFilterPayload {
    pub assignee: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}