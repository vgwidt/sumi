use uuid::Uuid;

use super::{request_delete, request_get, request_post, request_put};
use crate::types::*;

//get all users
pub async fn get_users() -> Result<Vec<UserRepresentation>, Error> {
    request_get::<Vec<UserRepresentation>>(format!("/users")).await
}

//get username from user_id
pub async fn get_userinfo(user_id: Uuid) -> Result<UserRepresentation, Error> {
    request_get(format!("/users/{}", user_id)).await
}

//get user preferences (relies on session in backend)
pub async fn get_user_preferences() -> Result<UserPreferences, Error> {
    request_get(format!("/preferences")).await
}

pub async fn update_user_preferences(
    user_preferences: UserPreferences,
) -> Result<UserPreferences, Error> {
    request_put::<UserPreferences, UserPreferences>(format!("/preferences"), user_preferences).await
}

pub async fn create(register_info: RegisterInfo) -> Result<UserInfo, Error> {
    request_post::<RegisterInfo, UserInfo>("/users".to_string(), register_info).await
}

pub async fn save(id: Uuid, user_update_info: UserUpdateInfo) -> Result<UserInfo, Error> {
    request_put::<UserUpdateInfo, UserInfo>(format!("/users/{}", id.to_string()), user_update_info)
        .await
}

pub async fn delete_user(user_id: Uuid) -> Result<(), Error> {
    request_delete(format!("/users/{}", user_id)).await
}
