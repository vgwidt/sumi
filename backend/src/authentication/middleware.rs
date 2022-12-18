use crate::models::session::TypedSession;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::HttpResponse;
use actix_web::{FromRequest, HttpMessage};
use actix_web_lab::middleware::Next;
use std::ops::Deref;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct UserId(Uuid);

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for UserId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    // Allow requests to login
    if req.path() == "/api/login" {
        return next.call(req).await;
    }

    let session = {
        let (http_request, payload) = req.parts_mut();
        TypedSession::from_request(http_request, payload).await
    }?;

    match session
        .get_user_id()
        .map_err(actix_web::error::ErrorInternalServerError)?
    {
        Some(user_id) => {
            req.extensions_mut().insert(UserId(user_id));
            next.call(req).await
        }
        None => {
            let response = HttpResponse::Unauthorized().finish();
            let e = anyhow::anyhow!("Not logged in");
            Err(InternalError::from_response(e, response).into())
        }
    }
}
