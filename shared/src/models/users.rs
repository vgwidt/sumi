use serde::{Serialize, Deserialize};
use uuid::Uuid;

//used by components to map user_id to display_name
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UserDisplay {
    pub user_id: Uuid,
    pub display_name: String,
}