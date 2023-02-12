use super::UserRepresentation;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TicketStatus {
    Open,
    InProgress,
    OnHold,
    Closed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct TicketInfo {
    pub ticket_id: i32,
    pub title: String,
    pub assignee: Option<UserRepresentation>,
    pub contact: Option<Uuid>,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub priority: String,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub revision: chrono::NaiveDateTime,
    pub revision_by: Option<Uuid>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TicketInfoWrapper {
    pub ticket: TicketInfo,
}

//Arbitrarily decided to user a wrapper for list of tickets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct TicketListInfo {
    pub tickets: Vec<TicketInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketCreateInfo {
    pub title: String,
    pub description: String,
    pub assignee: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub priority: String,
    pub status: String,
}

impl TicketCreateInfo {
    pub fn default() -> Self {
        Self {
            title: "".to_string(),
            description: "".to_string(),
            assignee: None,
            contact: None,
            priority: "".to_string(),
            status: "Open".to_string(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketUpdateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contact: Option<Option<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TicketStatusInfo {
    pub status: String,
}

//impl TicketStatus {
//     pub fn to_string(&self) -> String {
//         match self {
//             TicketStatus::Open => "Open".to_string(),
//             TicketStatus::InProgress => "In Progress".to_string(),
//             TicketStatus::OnHold => "On Hold".to_string(),
//             TicketStatus::Closed => "Closed".to_string(),
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct TicketStatusInfo {
//     pub status: TicketStatus,
// }

// impl TicketStatusInfo {
//     pub fn toggle(&self) -> String {
//         match self.status {
//             TicketStatus::Open => TicketStatus::Closed.to_string(),
//             TicketStatus::InProgress => TicketStatus::Closed.to_string(),
//             TicketStatus::OnHold => TicketStatus::Closed.to_string(),
//             TicketStatus::Closed => TicketStatus::Open.to_string(),
//         }
//     }

// }
