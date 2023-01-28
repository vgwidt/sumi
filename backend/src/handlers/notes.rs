use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{
    notes::{NewNote, Note, NotePayload, NoteRepresentation},
    users::User,
    Response,
};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/notes")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<NotePayload>,
) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let mut conn = pool.get()?;
        add_a_note(payload.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(NoteRepresentation::from)
            .collect::<Vec<NoteRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let note = note.first().unwrap();

    Ok(HttpResponse::Ok().json(note))
}

#[get("/notes")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let notes = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(NoteRepresentation::from)
            .collect::<Vec<NoteRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

#[get("/notes/{id}")]
async fn show(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let mut conn = pool.get()?;
        find_by_id(id.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(NoteRepresentation::from)
            .collect::<Vec<NoteRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let note = note.first().unwrap();

    Ok(HttpResponse::Ok().json(note))
}

//all notes for a ticket
#[get("/tickets/{id}/notes")]
async fn ticket_notes(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let notes = web::block(move || {
        let mut conn = pool.get()?;
        find_by_ticket_id(id.into_inner(), &mut conn)
    })
    .await?
    .map(|x| {
        x.into_iter()
            .map(NoteRepresentation::from)
            .collect::<Vec<NoteRepresentation>>()
    })
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(notes))
}

#[put("/notes/{id}")]
async fn update(
    id: web::Path<Uuid>,
    payload: web::Json<NotePayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let note = web::block(move || {
        let mut conn = pool.get()?;
        update_note(id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    

    Ok(HttpResponse::Ok().json(note))
}

#[delete("/notes/{id}")]
async fn delete(id: web::Path<Uuid>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_note(id.into_inner(), &mut conn)
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

fn add_a_note(
    payload: NotePayload,
    conn: &mut PgConnection,
) -> Result<Vec<(Note, Option<User>)>, DbError> {
    use crate::schema::notes::dsl::*;
    use crate::schema::users::dsl::users;

    let new_note = NewNote {
        note_id: Uuid::new_v4(),
        ticket: payload.ticket,
        owner: payload.owner,
        text: &payload.text,
        time: payload.time,
    };

    let result: Note = diesel::insert_into(notes)
        .values(&new_note)
        .get_result(conn)?;

    let note: Vec<(Note, Option<User>)> = notes
        .filter(note_id.eq(result.note_id))
        .left_join(users)
        .load::<(Note, Option<User>)>(conn)?;

    Ok(note)
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<(Note, Option<User>)>, DbError> {
    use crate::schema::notes::dsl::*;
    use crate::schema::users::dsl::users;

    let items: Vec<(Note, Option<User>)> =
        notes.left_join(users).load::<(Note, Option<User>)>(conn)?;

    Ok(items)
}

fn find_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Vec<(Note, Option<User>)>, DbError> {
    use crate::schema::notes::dsl::*;
    use crate::schema::users::dsl::users;

    let note: Vec<(Note, Option<User>)> =
        notes
            .filter(note_id.eq(&id))
            .left_join(users)
            .load::<(Note, Option<User>)>(conn)?;

    Ok(note)
}

fn update_note(
    id: Uuid,
    payload: NotePayload,
    conn: &mut PgConnection,
) -> Result<NoteRepresentation, DbError> {
    use crate::schema::notes::dsl::*;
    use crate::schema::users::dsl::users;

    let result: Note = diesel::update(notes.find(id))
        .set(text.eq(payload.text))
        .get_result(conn)?;

    let note: (Note, Option<User>) = notes
        .filter(note_id.eq(result.note_id))
        .left_join(users)
        .first::<(Note, Option<User>)>(conn)?;

    let note = NoteRepresentation::from(note);

    Ok(note)
}

fn delete_note(id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::notes::dsl::*;

    let count = diesel::delete(notes.find(id)).execute(conn)?;
    Ok(count)
}

fn find_by_ticket_id(
    id: i32,
    conn: &mut PgConnection,
) -> Result<Vec<(Note, Option<User>)>, DbError> {
    use crate::schema::notes::dsl::*;
    use crate::schema::users::dsl::users;

    let items: Vec<(Note, Option<User>)> =
        notes
            .filter(ticket.eq(&id))
            .left_join(users)
            .load::<(Note, Option<User>)>(conn)?;

    Ok(items)
}
