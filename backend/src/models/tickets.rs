use crate::schema::tickets;
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
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tickets)]
pub struct NewTicket<'a> {
    pub title: &'a str,
    pub assignee: Option<Uuid>,
    pub contact: Option<Uuid>,
    pub description: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub due_date: Option<chrono::NaiveDateTime>,
    pub priority: &'a str,
    pub status: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
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
pub struct TicketFilterPayload {
    pub assignee: Option<Uuid>,
    pub status: Option<String>,
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
}

impl From<(Ticket, Option<User>)> for TicketRepresentation {
    fn from(values: (Ticket, Option<User>)) -> Self {
        Self {
            ticket_id: values.0.ticket_id,
            title: values.0.title,
            assignee: Some(SomeUserRepresentation {
                user_id: Some(values.1.clone().unwrap_or_default().user_id),
                username: Some(values.1.clone().unwrap_or_default().username),
                display_name: Some(values.1.clone().unwrap_or_default().display_name),
                email: Some(values.1.clone().unwrap_or_default().email),
                created_at: Some(values.1.clone().unwrap_or_default().created_at),
                access: Some(values.1.clone().unwrap_or_default().access),
            }),
            contact: values.0.contact,
            description: values.0.description,
            created_at: values.0.created_at,
            updated_at: values.0.updated_at,
            due_date: values.0.due_date,
            priority: values.0.priority,
            status: values.0.status,
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
