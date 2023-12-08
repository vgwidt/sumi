use serde::{Deserialize, Serialize};

pub mod comments;
pub mod contacts;
pub mod documents;
pub mod notes;
pub mod session;
pub mod tickets;
pub mod users;
pub mod ticket_fields;

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}
