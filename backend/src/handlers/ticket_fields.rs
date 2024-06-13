use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use shared::models::response::Response;
use crate::models::ticket_fields::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/ticket_fields")]
async fn create_field(
    pool: web::Data<DbPool>,
    payload: web::Json<NewTicketCustomField>,
) -> Result<HttpResponse, Error> {

    let field = web::block(move || {
        let mut conn = pool.get()?;
        create_field_query(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = Response {
        success: true,
        message: None,
        data: Some(field),
    };

    Ok(HttpResponse::Ok().json(response))
}

fn create_field_query(
    payload: NewTicketCustomField,
    conn: &mut PgConnection,
) -> Result<TicketCustomField, DbError> {
    use crate::schema::ticket_custom_fields::dsl::*;

    diesel::insert_into(ticket_custom_fields)
        .values(&payload)
        .get_result(conn)
        .map_err(|e| Box::new(e) as DbError)
}

//Get all fields
#[get("/ticket_fields")]
async fn get_fields(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let fields = web::block(move || {
        let mut conn = pool.get()?;
        get_fields_query(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(fields))
}

fn get_fields_query(
    conn: &mut PgConnection,
) -> Result<Vec<TicketCustomField>, DbError> {
    use crate::schema::ticket_custom_fields::dsl::*;

    ticket_custom_fields
        .load::<TicketCustomField>(conn)
        .map_err(|e| Box::new(e) as DbError)
}

//Get specific field
#[get("/ticket_fields/{field_id}")]
async fn get_field(
    pool: web::Data<DbPool>,
    field_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let field = web::block(move || {
        let mut conn = pool.get()?;
        get_field_query(field_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(field))
}

fn get_field_query(
    field_id: i32,
    conn: &mut PgConnection,
) -> Result<TicketCustomField, DbError> {
    use crate::schema::ticket_custom_fields::dsl::*;

    ticket_custom_fields
        .filter(id.eq(field_id))
        .first::<TicketCustomField>(conn)
        .map_err(|e| Box::new(e) as DbError)
}

//Update field
#[put("/ticket_fields/{field_id}")]
async fn update_field(
    pool: web::Data<DbPool>,
    field_id: web::Path<i32>,
    payload: web::Json<TicketCustomFieldPayload>,
) -> Result<HttpResponse, Error> {

    let field = web::block(move || {
        let mut conn = pool.get()?;
        update_field_query(field_id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(field))
}

fn update_field_query(
    field_id: i32,
    payload: TicketCustomFieldPayload,
    conn: &mut PgConnection,
) -> Result<TicketCustomField, DbError> {
    use crate::schema::ticket_custom_fields::dsl::*;

    diesel::update(ticket_custom_fields.filter(id.eq(field_id)))
        .set(&payload)
        .get_result(conn)
        .map_err(|e| Box::new(e) as DbError)
}

//Delete field
#[delete("/ticket_fields/{field_id}")]
async fn delete_field(
    pool: web::Data<DbPool>,
    field_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {

    let field = web::block(move || {
        let mut conn = pool.get()?;
        delete_field_query(field_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(field))
}

fn delete_field_query(
    field_id: i32,
    conn: &mut PgConnection,
) -> Result<TicketCustomField, DbError> {
    use crate::schema::ticket_custom_fields::dsl::*;

    diesel::delete(ticket_custom_fields.filter(id.eq(field_id)))
        .get_result(conn)
        .map_err(|e| Box::new(e) as DbError)
}
