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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DocumentCreateInfo {
    pub parent_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub created_by: Uuid,
    pub updated_by: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DocumentUpdateInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<Option<Uuid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub version: Option<chrono::NaiveDateTime>,
}
