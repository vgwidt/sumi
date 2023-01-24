use super::super::DbPool;

use actix_session::Session;
use actix_web::{get, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::users::{UpdateUserPreferences, UserPreferences, UserPreferencesRepresentation};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/preferences")]
async fn get_preferences(pool: web::Data<DbPool>, session: Session) -> Result<HttpResponse, Error> {
    let id: Option<Uuid> = session.get("user_id")?;
    if let Some(id) = id {
        let user_preferences = web::block(move || {
            let mut conn = pool.get()?;
            get_user_preferences(id, &mut conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(user_preferences))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

#[put("/preferences")]
async fn update_preferences(
    pool: web::Data<DbPool>,
    session: Session,
    payload: web::Json<UpdateUserPreferences>,
) -> Result<HttpResponse, Error> {
    let id: Option<Uuid> = session.get("user_id")?;
    if let Some(id) = id {
        let user_preferences = web::block(move || {
            let mut conn = pool.get()?;
            update_user_preferences(id, payload.into_inner(), &mut conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(user_preferences))
    } else {
        Ok(HttpResponse::Unauthorized().finish())
    }
}

pub fn get_user_preferences(
    id: Uuid,
    conn: &mut PgConnection,
) -> Result<UserPreferencesRepresentation, DbError> {
    use crate::schema::user_preferences::dsl::*;

    let preferences: UserPreferences = user_preferences.filter(user_id.eq(id)).first(conn)?;

    let preferences = UserPreferencesRepresentation {
        theme: preferences.theme,
        locale: preferences.locale,
        timezone: preferences.timezone,
        custom_views: preferences.custom_views,
    };

    Ok(preferences)
}

fn update_user_preferences(
    id: Uuid,
    payload: UpdateUserPreferences,
    conn: &mut PgConnection,
) -> Result<UserPreferences, DbError> {
    use crate::schema::user_preferences::dsl::*;


    let preferences = diesel::update(user_preferences.filter(user_id.eq(id)))
        .set(&payload)
        .get_result::<UserPreferences>(conn)?;

    Ok(preferences)
}