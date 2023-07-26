use crate::models::users::User;
use actix_web::web;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use secrecy::{ExposeSecret, Secret};

use super::super::DbPool;
use diesel::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

pub fn get_stored_credentials(
    user: &str,
    pool: &DbPool,
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    use crate::schema::users::dsl::*;

    let mut conn = pool
        .get()
        .context("Failed to get DB connection from pool")?;

    let row = users
        .filter(username.eq(user))
        .first::<User>(&mut conn)
        .map_err(|e| {
            if e == diesel::NotFound {
                AuthError::InvalidCredentials(anyhow::anyhow!("Invalid credentials."))
            } else {
                AuthError::UnexpectedError(anyhow::anyhow!(e))
            }
        })?;

    let row = Some((row.user_id, Secret::new(row.password_hash)));
    Ok(row)
}

pub async fn validate_credentials(
    credentials: Credentials,
    pool: &DbPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new("".to_string());

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool)
            .context("Failed to get stored credentials")?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    web::block(move || verify_password_hash(credentials.password, expected_password_hash))
        .await
        .context("Failed to spawn blocking task")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unknown username"))
        .map_err(AuthError::InvalidCredentials)
}

fn verify_password_hash(
    password_candidate: Secret<String>,
    rec_expected_password_hash: Secret<String>,
) -> Result<(), AuthError> {
    

    let expected_password_hash: PasswordHash = match PasswordHash::new(rec_expected_password_hash.expose_secret()) {
        Ok(hash) => hash,
        Err(e) => {
            return Err(AuthError::InvalidCredentials(e.into()));
        }
    };

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .map_err(|_| AuthError::InvalidCredentials(anyhow::anyhow!("Invalid password")))
}

// ensures that new password meets requirements
pub fn check_password_reqs(password: &Secret<String>) -> Result<(), AuthError> {
    let min_length = 6;
    if password.expose_secret().len() < min_length {
        return Err(AuthError::InvalidCredentials(anyhow::anyhow!(
            "Password must be at least {} characters long",
            min_length
        )));
    }
    Ok(())
}

pub fn compute_password_hash(
    password: Secret<String>,
) -> Result<Secret<String>, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)?
    .to_string();
    Ok(Secret::new(password_hash))
}
