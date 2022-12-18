use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct UserRepresentation {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub access: String,
}

//user preferences
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserPreferences {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timezone: Option<String>,
}
