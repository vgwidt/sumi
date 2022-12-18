use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RegisterInfo {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub access: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterInfoWrapper {
    pub user: RegisterInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub access: String,
}

impl MyUser {
    pub fn is_authenticated(&self) -> bool {
        self.user_id != uuid::Uuid::nil()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserUpdateInfo {
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub access: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserUpdateInfoWrapper {
    pub user: UserUpdateInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
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
}
