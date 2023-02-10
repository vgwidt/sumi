use uuid::Uuid;

use super::{request_delete, request_get, request_post, request_put};
use crate::types::*;

//get list of documents for document tree
pub async fn get_doc_tree() -> Result<Vec<DocumentMetadata>, Error> {
    let results = request_get::<Vec<DocumentMetadata>>(format!("/documents")).await?;

    //sort in alphabetical order
    let mut sorted_results = results;
    sorted_results.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

    Ok(sorted_results)
}

//get document info
pub async fn get_document(document_id: &Uuid) -> Result<DocumentInfo, Error> {
    request_get::<DocumentInfo>(format!("/documents/{}", document_id)).await
}

//create document
pub async fn create_document(document: DocumentCreateUpdateInfo) -> Result<Response<DocumentInfo>, Error> {
    request_post::<DocumentCreateUpdateInfo, Response<DocumentInfo>>(format!("/documents"), document).await
}

//update document
pub async fn update_document(
    document_id: &Uuid,
    document: DocumentCreateUpdateInfo,
) -> Result<Response<DocumentInfo>, Error> {
    request_put::<DocumentCreateUpdateInfo, Response<DocumentInfo>>(
        format!("/documents/{}", document_id),
        document,
    )
    .await
}

pub async fn delete_document(document_id: Uuid) -> Result<SuccessResponse, Error> {
    request_delete::<SuccessResponse>(format!("/documents/{}", document_id)).await
}
