use crate::schema::{ticket_events, ticket_revisions, tickets};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::users::User;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Ticket {
    pub ticket_id: i32,
    pub assignee: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: String,
    pub status: String,
    pub resolution: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub revision: chrono::NaiveDateTime,
    pub revision_by: Option<Uuid>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tickets)]
pub struct NewTicket {
    pub title: String,
    pub assignee: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: String,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub revision: chrono::NaiveDateTime,
    pub revision_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = tickets)]
pub struct TicketPayload {
    pub title: String,
    pub assignee: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub description: String,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketUpdatePayload {
    pub title: Option<String>,
    pub assignee: Option<String>,
    pub contact: Option<String>,
    pub description: Option<String>,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub version: Option<chrono::NaiveDateTime>,
}

//Struct used to update ticket in database
#[derive(Debug, Insertable, AsChangeset, Deserialize, Default)]
#[diesel(table_name = tickets)]
pub struct UpdateTicket {
    pub title: Option<String>,
    pub assignee: Option<Option<Uuid>>,
    pub contact: Option<Option<Uuid>>,
    pub description: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: Option<String>,
    pub status: Option<String>,
    pub revision: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct TicketWrapper {
    pub tickets: Vec<TicketRepresentation>,
    pub page: i64,
    pub total_pages: i64,
    pub total_results: i64,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct TicketRepresentation {
    pub ticket_id: i32,
    pub title: String,
    pub assignee: Option<SomeUserRepresentation>,
    pub contact: Option<Uuid>,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: String,
    pub status: String,
    pub revision: chrono::NaiveDateTime,
}

impl From<(Ticket, Option<User>)> for TicketRepresentation {
    fn from(values: (Ticket, Option<User>)) -> Self {
        Self {
            ticket_id: values.0.ticket_id,
            title: values.0.title,
            assignee: {
                if let Some(user) = values.1 {
                    Some(SomeUserRepresentation {
                        user_id: Some(user.user_id),
                        username: Some(user.username),
                        display_name: Some(user.display_name),
                        email: Some(user.email),
                        created_at: Some(user.created_at),
                        access: Some(user.access),
                    })
                } else {
                    None
                }
            },
            contact: values.0.contact,
            description: values.0.description,
            created_at: values.0.created_at,
            updated_at: values.0.updated_at,
            due_date: values.0.due_date,
            priority: values.0.priority,
            status: values.0.status,
            revision: values.0.revision,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct SomeUserRepresentation {
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub access: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TicketRevision {
    pub revision_id: Uuid,
    pub ticket_id: i32,
    pub content: String,
    pub updated_by: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = ticket_revisions)]
pub struct NewTicketRevision {
    pub revision_id: Uuid,
    pub ticket_id: i32,
    pub description: String,
    pub updated_by: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
}

//Ticket events
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TicketEvent {
    pub event_id: Uuid,
    pub ticket_id: i32,
    pub event_type: String,
    pub event_data: String,
    pub user_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
}

//New ticket event
#[derive(Debug, Insertable)]
#[diesel(table_name = ticket_events)]
pub struct NewTicketEvent {
    pub event_id: Uuid,
    pub ticket_id: i32,
    pub event_type: String,
    pub event_data: String,
    pub user_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
}
