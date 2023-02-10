use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TicketEventType {
    Archived,
    Unarchived,
    Assigned,
    StatusChanged,
    PriorityChanged,
}

//impl to and from string for TicketEventType
impl TicketEventType {
    pub fn to_string(&self) -> String {
        match self {
            TicketEventType::Archived => "Archived".to_string(),
            TicketEventType::Unarchived => "Unarchived".to_string(),
            TicketEventType::Assigned => "Assigned".to_string(),
            TicketEventType::StatusChanged => "StatusChanged".to_string(),
            TicketEventType::PriorityChanged => "PriorityChanged".to_string(),
        }
    }
    pub fn from_string(s: &str) -> Result<TicketEventType, String> {
        match s {
            "Archived" => Ok(TicketEventType::Archived),
            "Unarchived" => Ok(TicketEventType::Unarchived),
            "Assigned" => Ok(TicketEventType::Assigned),
            "StatusChanged" => Ok(TicketEventType::StatusChanged),
            "PriorityChanged" => Ok(TicketEventType::PriorityChanged),
            _ => Err(format!("{} is not a valid TicketEventType", s)),
        }
    }
}
