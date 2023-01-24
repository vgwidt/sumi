use super::super::DbPool;

use actix_session::Session;
use actix_web::{delete, error::InternalError, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use secrecy::ExposeSecret;
use uuid::Uuid;

use crate::models::session::TypedSession;
use crate::schema::user_preferences::custom_views;
use crate::{
    authentication::{check_password_reqs, compute_password_hash},
    models::users::{
        MyUser, NewUser, UpdateUser, User, UserPayload, UserRepresentation, UserUpdatePayload,
    },
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/users")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<UserPayload>,
) -> Result<HttpResponse, Error> {
    let email_ok = validate_email(&payload.email);

    if email_ok.is_ok() {
        let user = web::block(move || {
            let mut conn = pool.get()?;
            add_a_user(payload.into_inner(), &mut conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(user))
    } else {
        //return error from validate_email fn
        email_ok.map_err(|e| InternalError::new(e, actix_web::http::StatusCode::BAD_REQUEST))?;
        Ok(HttpResponse::Ok().json("Internal Error"))
    }
}

#[get("/users")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn show(user_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(user_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let user_info: UserRepresentation = UserRepresentation {
        user_id: user.user_id,
        username: user.username,
        display_name: user.display_name,
        email: user.email,
        created_at: user.created_at,
        access: user.access,
    };

    Ok(HttpResponse::Ok().json(user_info))
}

//Returns user info for logged in user
#[get("/whoami")]
async fn whoami(pool: web::Data<DbPool>, session: TypedSession) -> Result<HttpResponse, Error> {
    let id: Option<Uuid> = match session.get_user_id() {
        Ok(id) => id,
        Err(_) => {
            return Err(InternalError::from_response(
                "Unauthorized",
                HttpResponse::Unauthorized().finish(),
            )
            .into())
        }
    };

    if let Some(id) = id {
        let user = web::block(move || {
            let mut conn = pool.get()?;
            get_my_info(id, &mut conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(
            InternalError::from_response("Unauthorized", HttpResponse::Unauthorized().finish())
                .into(),
        )
    }
}

#[put("/users/{id}")]
async fn update(
    user_id: web::Path<Uuid>,
    payload: web::Json<UserUpdatePayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    if let Some(email) = &payload.email {
        let email_ok = validate_email(email);
        if email_ok.is_err() {
            email_ok
                .map_err(|e| InternalError::new(e, actix_web::http::StatusCode::BAD_REQUEST))?;
            return Ok(HttpResponse::Ok().json("Internal Error"));
        }
    }

    let user = web::block(move || {
        let mut conn = pool.get()?;
        update_user(user_id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn destroy(
    id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let session_id: Option<Uuid> = session.get("user_id")?;

    //prevent user from deleting themselves
    if let Some(session_id) = session_id {
        let target_id = id.clone();
        if session_id == target_id {
            return Err(InternalError::from_response(
                "You cannot delete yourself",
                HttpResponse::Unauthorized().finish(),
            )
            .into());
        }
    }

    let user = web::block(move || {
        let mut conn = pool.get()?;
        delete_user(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

fn add_a_user(payload: UserPayload, conn: &mut PgConnection) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    //check password reqs
    match check_password_reqs(&payload.password) {
        Ok(_) => (),
        Err(e) => return Err(Box::new(e)),
    }

    let hash = match compute_password_hash(payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            return Err(e.into());
        }
    };

    let new_user = NewUser {
        user_id: Uuid::new_v4(),
        username: &payload.username,
        display_name: &payload.display_name,
        email: &payload.email,
        access: &payload.access,
        created_at: chrono::Utc::now().naive_utc(),
        password_hash: hash.expose_secret(),
    };

    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;

    //Create user preferences
    let result = diesel::sql_query("INSERT INTO user_preferences (user_id) VALUES ($1)")
        .bind::<diesel::sql_types::Uuid, _>(inserted_user.user_id)
        .execute(conn);

    match result {
        Ok(_) => Ok(inserted_user),
        Err(e) => Err(Box::new(e)),
    }
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<UserRepresentation>, DbError> {
    use crate::schema::users::dsl::*;

    let user_results = users.load::<User>(conn)?;

    let mut user_info_list: Vec<UserRepresentation> = Vec::new();

    for user in user_results {
        let user_info: UserRepresentation = UserRepresentation {
            user_id: user.user_id,
            username: user.username,
            display_name: user.display_name,
            email: user.email,
            created_at: user.created_at,
            access: user.access,
        };

        user_info_list.push(user_info);
    }

    Ok(user_info_list)
}

pub fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(user_id.eq(id)).first::<User>(conn)?;

    Ok(user)
}

pub fn get_my_info(id: Uuid, conn: &mut PgConnection) -> Result<MyUser, DbError> {
    use crate::schema::user_preferences::dsl::user_preferences;
    use crate::schema::user_preferences::{locale, theme, timezone};
    use crate::schema::users::dsl::*;

    let user = users
        .inner_join(user_preferences)
        .filter(user_id.eq(id))
        .select((
            user_id,
            username,
            display_name,
            email,
            created_at,
            access,
            theme,
            locale,
            timezone,
            custom_views,
        ))
        .first::<MyUser>(conn)?;

    Ok(user)
}

fn update_user(
    id: Uuid,
    user: UserUpdatePayload,
    conn: &mut PgConnection,
) -> Result<UserRepresentation, DbError> {
    use crate::schema::users::dsl::*;

    let mut update_user = UpdateUser {
        username: None,
        display_name: None,
        email: None,
        access: None,
        password_hash: None,
    };

    if let Some(new_username) = user.username {
        update_user.username = Some(new_username);
    }
    if let Some(new_display_name) = user.display_name {
        update_user.display_name = Some(new_display_name);
    }
    if let Some(new_email) = user.email {
        update_user.email = Some(new_email);
    }
    if let Some(new_access) = user.access {
        update_user.access = Some(new_access);
    }

    if let Some(password) = user.password {
        match check_password_reqs(&password) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e)),
        };
        let hash = compute_password_hash(password)?;
        update_user.password_hash = Some(hash.expose_secret().to_string());
    }

    let updated_user = diesel::update(users.filter(user_id.eq(id)))
        .set(&update_user)
        .get_result::<User>(conn)?;

    //dont return the password hash
    let user_info: UserRepresentation = UserRepresentation {
        user_id: updated_user.user_id,
        username: updated_user.username,
        display_name: updated_user.display_name,
        email: updated_user.email,
        created_at: updated_user.created_at,
        access: updated_user.access,
    };

    Ok(user_info)
}

fn delete_user(id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::users::dsl::*;

    let deleted_user = diesel::delete(users.find(id)).execute(conn)?;

    Ok(deleted_user)
}

fn validate_email(email: &str) -> Result<(), Error> {
    if email.is_empty() {
        Err(actix_web::error::ErrorBadRequest("Email cannot be empty"))
    } else if !email.contains("@") {
        Err(actix_web::error::ErrorBadRequest("Email must contain an @"))
    } else if email.len() > 255 {
        Err(actix_web::error::ErrorBadRequest(
            "Email must be less than 255 characters",
        ))
    } else {
        Ok(())
    }
}
