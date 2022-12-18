use uuid::Uuid;

use super::{request_delete, request_get, request_post};
use crate::types::*;

pub async fn create(note: NoteCreateInfo) -> Result<NoteInfo, Error> {
    request_post::<NoteCreateInfo, NoteInfo>("/notes".to_owned(), note).await
}

pub async fn delete_note(note_id: Uuid) -> Result<SuccessResponse, Error> {
    request_delete::<SuccessResponse>(format!("/notes/{}", note_id)).await
}

pub async fn get_by_id(note_id: Uuid) -> Result<NoteInfo, Error> {
    request_get::<NoteInfo>(format!("/notes/{}", note_id)).await
}
