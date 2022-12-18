use super::{request_get, request_post};
use crate::types::*;

//Get current user info
pub async fn current() -> Result<MyUser, Error> {
    request_get::<MyUser>("/whoami".to_string()).await
}

pub async fn login(login_info: &LoginInfo) -> Result<SuccessResponse, Error> {
    let response =
        request_post::<&LoginInfo, SuccessResponse>("/login".to_string(), login_info).await;

    response
}

pub async fn logout() -> Result<(), Error> {
    request_post::<(), ()>("/logout".to_string(), ()).await
}
