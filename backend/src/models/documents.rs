use crate::schema::document_revisions;
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
pub struct DocumentCreatePayload {
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentUpdatePayload {
    pub parent_id: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub archived: Option<bool>,
    pub version: Option<chrono::NaiveDateTime>,
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

//values for creating revision
#[derive(Debug, Insertable)]
#[diesel(table_name = document_revisions)]
pub struct NewDocumentRevision {
    pub revision_id: Uuid,
    pub document_id: Uuid,
    pub content: String,
    pub updated_by: Option<Uuid>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = documents)]
pub struct UpdateDocument {
    pub parent_id: Option<Option<Uuid>>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub updated_by: Option<Uuid>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub archived: Option<bool>,
}
