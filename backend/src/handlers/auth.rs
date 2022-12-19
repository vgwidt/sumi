use super::super::DbPool;
use crate::authentication::AuthError;
use crate::authentication::{validate_credentials, Credentials};
use crate::models::session::TypedSession;
use crate::models::Response;
use actix_web::error::InternalError;
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web::{post, web};
type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/login")]
pub async fn login(
    pool: web::Data<DbPool>,
    credentials: web::Json<Credentials>,
    session: TypedSession,
) -> Result<HttpResponse, InternalError<LoginError>> {
    let credentials = credentials.into_inner();
    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            session.renew();
            session
                .insert_user_id(user_id)
                .map_err(|e| login_redirect(LoginError::UnexpectedError(e.into())))?;

            let response = Response {
                success: true,
                message: "Login successful".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let e = match e {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(e.into()),
            };
            let e = LoginError::UnexpectedError(e.into());

            let response = Response {
                success: false,
                message: e.to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

fn login_redirect(e: LoginError) -> InternalError<LoginError> {
    let body = format!("{{\"error\": \"{}\"}}", e.to_string());

    let response = HttpResponse::Ok()
        .insert_header((LOCATION, "/login"))
        .body(body);

    InternalError::from_response(e, response)
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

#[post("/logout")]
pub async fn logout(session: TypedSession) -> Result<HttpResponse, InternalError<DbError>> {
    session.log_out();

    //We could check to confirm if the session still exists or not and return a response accordingly

    let response = Response {
        success: true,
        message: "Logout successful".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}
