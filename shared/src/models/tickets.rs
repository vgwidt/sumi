use serde::{Deserialize, Serialize};

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
