use crate::schema::documents;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Document {
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub archived: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument<'a> {
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub url: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentPayload {
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DocumentRepresentation {
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub url: String,
    pub title: String,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub archived: bool,
}

//DocumentTreeInfo
#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct DocumentTreeInfo {
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub url: String,
    pub title: String,
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DocumentRevision {
    pub revision_id: Uuid,
    pub document_id: Uuid,
    pub content: String,
    pub updated_by: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
}