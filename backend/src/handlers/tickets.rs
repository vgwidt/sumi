use super::super::DbPool;

use actix_web::{delete, get, options, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::models::{
    tickets::{NewTicket, Ticket, TicketFilterPayload, TicketPayload, TicketRepresentation},
    users::User,
    Response,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize)]
struct TicketWrapper {
    tickets: Vec<TicketRepresentation>,
}

//options
#[options("/tickets")]
async fn options() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[post("/tickets")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<TicketPayload>,
) -> Result<HttpResponse, Error> {
    let ticket = web::block(move || {
        let mut conn = pool.get()?;
        add_a_ticket(payload.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket = ticket.first().unwrap();

    Ok(HttpResponse::Ok().json(ticket))
}

// All tickets with optional status filter (open, closed)
#[get("/tickets")]
async fn index(
    pool: web::Data<DbPool>,
    query: web::Query<TicketFilterPayload>,
) -> Result<HttpResponse, Error> {
    let tickets = web::block(move || {
        let mut conn = pool.get()?;
        find(&mut conn, Some(query.into_inner()))
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket_list: TicketWrapper = TicketWrapper { tickets };

    Ok(HttpResponse::Ok().json(ticket_list))
}

#[get("/tickets/assignee/{assignee}")]
async fn by_assignee(
    pool: web::Data<DbPool>,
    assignee: web::Path<String>,
) -> Result<HttpResponse, Error> {
    //convert assignee to Uuid
    let assignee_uuid = Uuid::parse_str(&assignee).unwrap();

    let tickets = web::block(move || {
        let mut conn = pool.get()?;
        find_by_user_id(assignee_uuid, &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket_list: TicketWrapper = TicketWrapper { tickets };

    Ok(HttpResponse::Ok().json(ticket_list))
}

//get tickets by user_id
#[get("/tickets?user_id={user_id}")]
async fn by_user_id(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    //convert to Uuid
    let id = Uuid::from_u128(id.into_inner() as u128);

    let tickets = web::block(move || {
        let mut conn = pool.get()?;
        find_by_user_id(id, &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(tickets))
}

#[get("/tickets/{id}")]
async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let ticket = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket = ticket.first().unwrap();

    Ok(HttpResponse::Ok().json(ticket))
}

#[put("/tickets/{id}")]
async fn update(
    id: web::Path<i32>,
    payload: web::Json<TicketPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let ticket = web::block(move || {
        let mut conn = pool.get()?;
        update_ticket(id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket = ticket.first().unwrap();

    Ok(HttpResponse::Ok().json(ticket))
}

#[delete("/tickets/{id}")]
async fn destroy(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_ticket(id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if result > 1 {
        let response = Response {
            success: true,
            message: "Note deleted".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            success: false,
            message: "Note not found".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}

fn add_a_ticket(
    payload: TicketPayload,
    conn: &mut PgConnection,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let new_ticket = NewTicket {
        title: &payload.title,
        assignee: payload.assignee,
        contact: payload.contact,
        description: &payload.description,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        due_date: payload.due_date,
        priority: &payload.priority,
        status: &payload.status,
    };

    let result: Ticket = diesel::insert_into(tickets)
        .values(&new_ticket)
        .get_result(conn)?;

    let ticket: Vec<(Ticket, Option<User>)> = tickets
        .filter(ticket_id.eq(result.ticket_id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(ticket)
}

fn find(
    conn: &mut PgConnection,
    filters: Option<TicketFilterPayload>,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let mut query = tickets.left_join(users).into_boxed();

    if let Some(filters) = filters {
        if let Some(tassignee) = filters.assignee {
            if tassignee == Uuid::nil() {
                query = query.filter(assignee.is_null());
            } else {
                query = query.filter(assignee.eq(tassignee));
            }
        }

        if let Some(tstatus) = filters.status {
            if tstatus == "open" || tstatus == "Open" {
                //anything but closed for now
                query = query.filter(status.ne("Closed"));
            } else if tstatus == "closed" || tstatus == "Closed" {
                query = query.filter(status.eq("Closed"));
            }
        }
    }

    let items = query.load::<(Ticket, Option<User>)>(conn)?;

    Ok(items)
}

fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let ticket: Vec<(Ticket, Option<User>)> = tickets
        .filter(ticket_id.eq(&id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(ticket)
}

fn update_ticket(
    id: i32,
    ticket: TicketPayload,
    conn: &mut PgConnection,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let result: Ticket = diesel::update(tickets.find(id))
        .set((
            title.eq(&ticket.title),
            assignee.eq(ticket.assignee),
            contact.eq(ticket.contact),
            description.eq(&ticket.description),
            updated_at.eq(chrono::Utc::now().naive_utc()),
            priority.eq(&ticket.priority),
            status.eq(&ticket.status),
        ))
        .get_result(conn)?;

    let ticket: Vec<(Ticket, Option<User>)> = tickets
        .filter(ticket_id.eq(result.ticket_id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(ticket)
}

fn delete_ticket(id: i32, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::tickets::dsl::*;

    let count = diesel::delete(tickets.find(id)).execute(conn)?;
    Ok(count)
}

fn find_by_user_id(
    id: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let items: Vec<(Ticket, Option<User>)> = tickets
        .filter(assignee.eq(&id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(items)
}
