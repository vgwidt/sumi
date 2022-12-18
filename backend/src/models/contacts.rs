use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::contacts;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Contact {
    pub contact_id: Uuid,
    pub display_name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = contacts)]
pub struct NewContact<'a> {
    pub display_name: &'a str,
    pub email: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPayload {
    pub display_name: String,
    pub email: String,
}
