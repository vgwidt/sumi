use super::super::DbPool;

use actix_web::{delete, error::InternalError, get, options, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;
use shared::models::{response::Response, tickets::TicketEventType};
use uuid::Uuid;

use crate::models::{
    session::TypedSession,
    tickets::{
        NewTicket, NewTicketEvent, NewTicketRevision, Ticket, TicketEvent, TicketFilterPayload,
        TicketPayload, TicketRepresentation, TicketRevision, TicketUpdatePayload, UpdateTicket,
    },
    users::User,
    SuccessResponse,
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
    session: TypedSession,
) -> Result<HttpResponse, Error> {
    let time = chrono::Utc::now().naive_utc();
    let user_id: Option<Uuid> = match session.get_user_id() {
        Ok(id) => id,
        Err(_) => {
            return Err(InternalError::from_response(
                "Unauthorized",
                HttpResponse::Unauthorized().finish(),
            )
            .into())
        }
    };

    let new_ticket = NewTicket {
        title: payload.title.clone(),
        assignee: payload.assignee,
        contact: payload.contact,
        description: payload.description.clone(),
        created_at: time.clone(),
        updated_at: time.clone(),
        due_date: payload.due_date,
        priority: payload.priority.clone(),
        status: payload.status.clone(),
        created_by: user_id.clone(),
        updated_by: user_id.clone(),
        revision: time.clone(),
        revision_by: user_id.clone(),
    };

    let ticket = web::block(move || {
        let mut conn = pool.get()?;
        add_a_ticket(new_ticket, &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket = ticket.first().unwrap();

    let response = Response {
        success: true,
        message: None,
        data: Some(ticket),
    };

    Ok(HttpResponse::Ok().json(response))
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
    payload: web::Json<TicketUpdatePayload>,
    pool: web::Data<DbPool>,
    session: TypedSession,
) -> Result<HttpResponse, Error> {
    let user_id: Option<Uuid> = match session.get_user_id() {
        Ok(id) => id,
        Err(_) => {
            return Err(InternalError::from_response(
                "Unauthorized",
                HttpResponse::Unauthorized().finish(),
            )
            .into())
        }
    };

    let old_ticket: Ticket = {
        let pool = pool.clone();
        let id = id.clone();
        web::block(move || {
            let mut conn = pool.get()?;
            get_ticket_by_id(id, &mut conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?
    };

    let time = chrono::Utc::now().naive_utc();

    let mut updated_ticket = UpdateTicket {
        title: payload.title.clone(),
        assignee: payload.assignee,
        contact: payload.contact,
        description: payload.description.clone(),
        due_date: payload.due_date,
        priority: payload.priority.clone(),
        status: payload.status.clone(),
        updated_at: Some(time.clone()),
        revision: if payload.description.is_some() {
            Some(time.clone())
        } else {
            None
        },
    };

    if updated_ticket.description.is_some() {
        //Set payload to none if content is the same (to prevent revision and timestamp update) otherwise proceed
        if updated_ticket.description.clone().unwrap() == old_ticket.description {
            updated_ticket.description = None;
        } else {
            //If version is in the payload, it means check revision
            if let Some(version) = payload.version {
                //Get the latest revision
                if version != old_ticket.revision {
                    let response: Response<Ticket> = Response {
                        success: false,
                        message: Some("Ticket description is out of date".to_string()),
                        data: None,
                    };
                    return Ok(HttpResponse::Ok().json(response));
                }
            }

            //create revision from old document
            let revision = NewTicketRevision {
                revision_id: Uuid::new_v4(),
                ticket_id: old_ticket.ticket_id,
                description: old_ticket.description,
                updated_by: old_ticket.updated_by,
                updated_at: old_ticket.updated_at,
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_ticket_revision(revision, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    //For each status, priority, assignee, and title change, check if it is the same as old ticket and if not create an event for each
    if payload.status.is_some() {
        if payload.status.clone().unwrap() != old_ticket.status {
            let event = NewTicketEvent {
                event_id: Uuid::new_v4(),
                ticket_id: old_ticket.ticket_id,
                event_type: TicketEventType::StatusUpdated.to_string(),
                event_data: payload.status.clone().unwrap(),
                user_id: user_id.clone(),
                created_at: time.clone(),
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_ticket_event(event, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    if payload.priority.is_some() {
        if payload.priority.clone().unwrap() != old_ticket.priority {
            let event = NewTicketEvent {
                event_id: Uuid::new_v4(),
                ticket_id: old_ticket.ticket_id,
                event_type: TicketEventType::PriorityUpdated.to_string(),
                event_data: payload.priority.clone().unwrap(),
                user_id: user_id.clone(),
                created_at: time.clone(),
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_ticket_event(event, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    if payload.assignee.is_some() {
        if payload.assignee.clone().unwrap() != old_ticket.assignee {
            let event = NewTicketEvent {
                event_id: Uuid::new_v4(),
                ticket_id: old_ticket.ticket_id,
                event_type: TicketEventType::Assigned.to_string(),
                event_data: if payload.assignee.clone().unwrap() == None {
                    "".to_string()
                } else {
                    payload.assignee.clone().unwrap().unwrap().to_string()
                },
                user_id: user_id.clone(),
                created_at: time.clone(),
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_ticket_event(event, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    if payload.title.is_some() {
        if payload.title.clone().unwrap() != old_ticket.title {
            let event = NewTicketEvent {
                event_id: Uuid::new_v4(),
                ticket_id: old_ticket.ticket_id,
                event_type: TicketEventType::TitleUpdated.to_string(),
                event_data: payload.title.clone().unwrap(),
                user_id: user_id.clone(),
                created_at: time.clone(),
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_ticket_event(event, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    let ticket = web::block(move || {
        let mut conn = pool.get()?;
        update_ticket(id.into_inner(), updated_ticket, &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(TicketRepresentation::from)
            .collect::<Vec<TicketRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let ticket = ticket.first().unwrap();

    let response = Response {
        success: true,
        message: None,
        data: Some(ticket),
    };

    Ok(HttpResponse::Ok().json(response))
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
        let response = SuccessResponse {
            success: true,
            message: "Note deleted".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = SuccessResponse {
            success: false,
            message: "Note not found".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}

#[get("/tickets/{id}/revisions")]
async fn revisions(
    ticket_id: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let revisions = web::block(move || {
        let mut conn = pool.get()?;
        get_ticket_revisions(ticket_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(revisions))
}

fn add_a_ticket(
    payload: NewTicket,
    conn: &mut PgConnection,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let result: Ticket = diesel::insert_into(tickets)
        .values(&payload)
        .get_result(conn)?;

    let ticket: Vec<(Ticket, Option<User>)> = tickets
        .filter(ticket_id.eq(result.ticket_id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(ticket)
}

#[get("/tickets/{id}/events")]
async fn events(ticket_id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let events = web::block(move || {
        let mut conn = pool.get()?;
        get_ticket_events(ticket_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(events))
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

/// Find ticket by id and join with user
fn find_by_id(id: i32, conn: &mut PgConnection) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let ticket: Vec<(Ticket, Option<User>)> = tickets
        .filter(ticket_id.eq(&id))
        .left_join(users)
        .load::<(Ticket, Option<User>)>(conn)?;

    Ok(ticket)
}

/// Find ticket by ID with no join
fn get_ticket_by_id(id: i32, conn: &mut PgConnection) -> Result<Ticket, DbError> {
    use crate::schema::tickets::dsl::*;

    let ticket = tickets.find(id).first::<Ticket>(conn)?;

    Ok(ticket)
}

fn update_ticket(
    id: i32,
    payload: UpdateTicket,
    conn: &mut PgConnection,
) -> Result<Vec<(Ticket, Option<User>)>, DbError> {
    use crate::schema::tickets::dsl::*;
    use crate::schema::users::dsl::users;

    let result: Ticket = diesel::update(tickets.find(id))
        .set(&payload)
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

fn create_ticket_revision(
    payload: NewTicketRevision,
    conn: &mut PgConnection,
) -> Result<TicketRevision, DbError> {
    use crate::schema::ticket_revisions::dsl::*;

    let result = diesel::insert_into(ticket_revisions)
        .values(&payload)
        .get_result::<TicketRevision>(conn)?;

    Ok(result)
}

fn get_ticket_revisions(id: i32, conn: &mut PgConnection) -> Result<Vec<TicketRevision>, DbError> {
    use crate::schema::ticket_revisions::dsl::*;

    let results = ticket_revisions
        .filter(ticket_id.eq(id))
        .load::<TicketRevision>(conn)?;

    Ok(results)
}

fn create_ticket_event(
    payload: NewTicketEvent,
    conn: &mut PgConnection,
) -> Result<TicketEvent, DbError> {
    use crate::schema::ticket_events::dsl::*;

    let result = diesel::insert_into(ticket_events)
        .values(&payload)
        .get_result::<TicketEvent>(conn)?;

    Ok(result)
}

fn get_ticket_events(id: i32, conn: &mut PgConnection) -> Result<Vec<TicketEvent>, DbError> {
    use crate::schema::ticket_events::dsl::*;

    let results = ticket_events
        .filter(ticket_id.eq(id))
        .load::<TicketEvent>(conn)?;

    Ok(results)
}
