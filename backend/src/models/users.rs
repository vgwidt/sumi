use crate::schema::{user_preferences, users};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Clone, Default)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub access: String,
    pub password_hash: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub user_id: Uuid,
    pub username: &'a str,
    pub display_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub access: &'a str,
    pub password_hash: &'a str,
}

#[derive(Debug, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub access: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct UserRepresentation {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub access: String,
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub access: String,
    pub password: secrecy::Secret<String>,
}

//Can get rid of this if we make UserPayload password optional and update respective code
#[derive(Debug, Deserialize)]
pub struct UserUpdatePayload {
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub access: Option<String>,
    pub password: Option<secrecy::Secret<String>>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MyUser {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub access: String,
    pub theme: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub custom_views: serde_json::Value,
}

// User Preferences
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = user_preferences)]
pub struct UserPreferences {
    pub user_id: Uuid,
    pub theme: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub custom_views: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferencesRepresentation {
    pub theme: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub custom_views: serde_json::Value,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = user_preferences)]
pub struct UpdateUserPreferences {
    pub theme: Option<String>,
    pub locale: Option<String>,
    pub timezone: Option<String>,
    pub custom_views: Option<serde_json::Value>,
}