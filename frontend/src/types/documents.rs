use serde::{Deserialize, Serialize};
use uuid::Uuid;

//For generating document tree
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DocumentMetadata {
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub url: String,
    pub title: String,
    pub archived: bool,
}

//individual document info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct DocumentInfo {
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

//Update document info. Note that url will be generated in backend
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DocumentCreateUpdateInfo {
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<chrono::NaiveDateTime>,
}

