use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::notes;

use super::{tickets::SomeUserRepresentation, users::User};

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct Note {
    pub note_id: Uuid,
    pub ticket: i32,
    pub owner: Option<Uuid>,
    pub text: String,
    pub time: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub note_id: Uuid,
    pub ticket: i32,
    pub owner: Option<Uuid>,
    pub text: &'a str,
    pub time: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotePayload {
    pub ticket: i32,
    pub owner: Option<Uuid>,
    pub text: String,
    pub time: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteRepresentation {
    pub note_id: Uuid,
    pub ticket: i32,
    pub owner: Option<SomeUserRepresentation>,
    pub text: String,
    pub time: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<(Note, Option<User>)> for NoteRepresentation {
    fn from(values: (Note, Option<User>)) -> Self {
        Self {
            note_id: values.0.note_id,
            ticket: values.0.ticket,
            owner: Some(SomeUserRepresentation {
                user_id: Some(values.1.clone().unwrap_or_default().user_id),
                username: Some(values.1.clone().unwrap_or_default().username),
                display_name: Some(values.1.clone().unwrap_or_default().display_name),
                email: Some(values.1.clone().unwrap_or_default().email),
                created_at: Some(values.1.clone().unwrap_or_default().created_at),
                access: Some(values.1.clone().unwrap_or_default().access),
            }),
            text: values.0.text,
            time: values.0.time,
            created_at: values.0.created_at,
            updated_at: values.0.updated_at,
        }
    }
}
