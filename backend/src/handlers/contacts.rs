use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::contacts::{Contact, ContactPayload, NewContact};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/contacts")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<ContactPayload>,
) -> Result<HttpResponse, Error> {
    let contact = web::block(move || {
        let mut conn = pool.get()?;
        add_a_contact(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(contact))
}

#[get("/contacts")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let contacts = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(contacts))
}

#[get("/contacts/{id}")]
async fn show(contact_id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let contact = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(contact_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(contact))
}

#[put("/contacts/{id}")]
async fn update(
    contact_id: web::Path<Uuid>,
    payload: web::Json<ContactPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let contact = web::block(move || {
        let mut conn = pool.get()?;
        update_contact(contact_id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(contact))
}

#[delete("/contacts/{id}")]
async fn delete(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let contact = web::block(move || {
        let mut conn = pool.get()?;
        delete_contact(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(contact))
}

fn add_a_contact(contact: ContactPayload, conn: &mut PgConnection) -> Result<Contact, DbError> {
    use crate::schema::contacts::dsl::*;

    let new_contact = NewContact {
        display_name: &contact.display_name,
        email: &contact.email,
    };

    let inserted_contact = diesel::insert_into(contacts)
        .values(&new_contact)
        .get_result(conn)?;

    Ok(inserted_contact)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Contact>, DbError> {
    use crate::schema::contacts::dsl::*;

    let all_contacts = contacts.load::<Contact>(conn)?;

    Ok(all_contacts)
}

fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Contact, DbError> {
    use crate::schema::contacts::dsl::*;

    let contact = contacts.filter(contact_id.eq(id)).first::<Contact>(conn)?;

    Ok(contact)
}

fn update_contact(
    id: Uuid,
    contact: ContactPayload,
    conn: &mut PgConnection,
) -> Result<Contact, DbError> {
    use crate::schema::contacts::dsl::*;

    let updated_contact = diesel::update(contacts.filter(contact_id.eq(id)))
        .set((
            display_name.eq(contact.display_name),
            email.eq(contact.email),
        ))
        .get_result(conn)?;

    Ok(updated_contact)
}

fn delete_contact(id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::contacts::dsl::*;

    let deleted_contact = diesel::delete(contacts.filter(contact_id.eq(id))).execute(conn)?;

    Ok(deleted_contact)
}
