use actix_web::{HttpResponse, error::InternalError};
use uuid::Uuid;

pub fn parse_uuid(input: &Option<String>) -> Result<Option<Option<Uuid>>, actix_web::Error> {
    match input {
        None => Ok(None),
        Some(value) => {
            if value.is_empty() {
                Ok(Some(None))
            } else {
                let result = Uuid::parse_str(value);
                match result {
                    Ok(uuid) => Ok(Some(Some(uuid))),
                    Err(_) => {
                        Err(InternalError::from_response(
                            "Invalid UUID",
                            HttpResponse::BadRequest().finish(),
                        )
                        .into())
                    }
                }
            }
        }
    }
}