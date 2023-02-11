use super::super::DbPool;

use actix_web::{delete, error::InternalError, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use shared::models::response::Response;
use uuid::Uuid;

use crate::models::{documents::*, session::TypedSession, SuccessResponse};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/documents")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<DocumentCreatePayload>,
) -> Result<HttpResponse, Error> {
    let document = web::block(move || {
        let mut conn = pool.get()?;
        create_document(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = Response {
        success: true,
        message: None,
        data: Some(document),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Handler for GET /documents, returns documents for generating tree
#[get("/documents")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let documents = web::block(move || {
        let mut conn = pool.get()?;
        get_document_list(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(documents))
}

#[get("/documents/{id}")]
async fn show(
    document_id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let document = web::block(move || {
        let mut conn = pool.get()?;
        get_document_by_id(document_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(document))
}

#[put("/documents/{id}")]
async fn update(
    document_id: web::Path<Uuid>,
    mut payload: web::Json<DocumentUpdatePayload>,
    pool: web::Data<DbPool>,
    session: TypedSession,
) -> Result<HttpResponse, Error> {
    let user_id: Option<Uuid> = match session.get_user_id() {
        Ok(id) => id,
        Err(_) => {
            return Err(InternalError::from_response(
                "Unauthorized",
                HttpResponse::Unauthorized().finish(),
            )
            .into())
        }
    };

    //If it contains content, it should mean a new revision
    if payload.content.is_some() {
        let old_document = {
            let pool = pool.clone();
            let document_id = document_id.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                get_document_by_id(document_id, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?
        };

        //Check if the content matches the old document, if so, set payload.content to None
        //This ensures that the timestamp will not be updated
        //Creation of revision will also be skipped of content hasn't changed
        //Otherwise we can safely continue, because content is the only thing that will result in revision
        if payload.content.clone().unwrap() == old_document.content {
            payload.content = None;
        } else {
            //If version is in the payload, it means check revision
            if let Some(version) = payload.version {
                //Get the latest revision
                if version != old_document.updated_at {
                    let response: Response<Document> = Response {
                        success: false,
                        message: Some("Document is out of date".to_string()),
                        data: None,
                    };
                    return Ok(HttpResponse::Ok().json(response));
                }
            }

            //create revision from old document
            let revision = NewDocumentRevision {
                revision_id: Uuid::new_v4(),
                document_id: old_document.document_id,
                content: old_document.content,
                updated_by: old_document.updated_by,
                updated_at: old_document.updated_at,
            };

            let pool = pool.clone();
            web::block(move || {
                let mut conn = pool.get()?;
                create_document_revision(revision, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    let document = web::block(move || {
        let mut conn = pool.get()?;
        update_document(
            document_id.into_inner(),
            payload.into_inner(),
            user_id,
            &mut conn,
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let response = Response {
        success: true,
        message: None,
        data: Some(document),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[delete("/documents/{id}")]
async fn delete(
    document_id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || {
        let mut conn = pool.get()?;
        delete_document(document_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if result > 1 {
        let response = SuccessResponse {
            success: true,
            message: "Document deleted".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = SuccessResponse {
            success: false,
            message: "Document not found".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    }
}

#[get("/documents/{id}/revisions")]
async fn revisions(
    document_id: web::Path<Uuid>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let revisions = web::block(move || {
        let mut conn = pool.get()?;
        get_document_revisions(document_id.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(revisions))
}

fn get_document_list(conn: &mut PgConnection) -> Result<Vec<DocumentTreeInfo>, DbError> {
    use crate::schema::documents::dsl::*;

    let results = documents
        .select((document_id, parent_id, url, title, archived))
        .load::<DocumentTreeInfo>(conn)?;

    Ok(results)
}

fn get_document_by_id(id: Uuid, conn: &mut PgConnection) -> Result<Document, DbError> {
    use crate::schema::documents::dsl::*;

    let result = documents.find(id).first::<Document>(conn)?;

    Ok(result)
}

fn create_document(
    payload: DocumentCreatePayload,
    conn: &mut PgConnection,
) -> Result<Document, DbError> {
    use crate::schema::documents::dsl::*;

    let mut adjusted_title = payload.title;
    if adjusted_title.is_empty() {
        adjusted_title = "Untitled".to_string();
    };

    let new_document = NewDocument {
        document_id: Uuid::new_v4(),
        parent_id: payload.parent_id,
        url: &generate_url(),
        title: &adjusted_title,
        content: &payload.content,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        created_by: payload.created_by,
        updated_by: payload.updated_by,
        archived: false,
    };

    let result = diesel::insert_into(documents)
        .values(&new_document)
        .get_result::<Document>(conn)?;

    Ok(result)
}

fn update_document(
    id: Uuid,
    payload: DocumentUpdatePayload,
    user_id: Option<Uuid>,
    conn: &mut PgConnection,
) -> Result<Document, DbError> {
    use crate::schema::documents::dsl::*;

    let adjusted_title = {
        if let Some(title_value) = payload.title {
            if title_value.is_empty() {
                Some("Untitled".to_string())
            } else {
                Some(title_value)
            }
        } else {
            None
        }
    };

    let doc = UpdateDocument {
        parent_id: payload.parent_id,
        title: adjusted_title,
        content: payload.content.clone(),
        updated_at: if payload.content.is_some() {
            Some(chrono::Utc::now().naive_utc())
        } else {
            None
        },
        updated_by: user_id,
        archived: payload.archived,
    };

    let result = diesel::update(documents.find(id))
        .set(&doc)
        .get_result::<Document>(conn)?;

    Ok(result)
}

fn delete_document(id: Uuid, conn: &mut PgConnection) -> Result<usize, DbError> {
    use crate::schema::documents::dsl::*;

    // Update the parent_id of children documents
    // Does the doc we are deleting have a parent_id set?
    let new_parent = documents
        .find(id)
        .select(parent_id)
        .first::<Option<Uuid>>(conn)?;

    //if it does, we need to set the parent_id of the children to the parent_id of the document we are deleting
    if new_parent.is_some() {
        diesel::update(documents.filter(parent_id.eq(id)))
            .set(parent_id.eq(new_parent))
            .execute(conn)?;
    }
    //otherwise set the parent_id of the children to null to put them at root level
    else {
        diesel::update(documents.filter(parent_id.eq(id)))
            .set(parent_id.eq::<Option<Uuid>>(None))
            .execute(conn)?;
    }

    let count = diesel::delete(documents.find(id)).execute(conn)?;

    Ok(count)
}

fn generate_url() -> String {
    //random 8-digit hex string
    let random_string: String =
        rand::Rng::sample_iter(rand::thread_rng(), &rand::distributions::Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

    random_string
}

fn create_document_revision(
    payload: NewDocumentRevision,
    conn: &mut PgConnection,
) -> Result<DocumentRevision, DbError> {
    use crate::schema::document_revisions::dsl::*;

    let result = diesel::insert_into(document_revisions)
        .values(&payload)
        .get_result::<DocumentRevision>(conn)?;

    Ok(result)
}

fn get_document_revisions(
    doc_id: Uuid,
    conn: &mut PgConnection,
) -> Result<Vec<DocumentRevision>, DbError> {
    use crate::schema::document_revisions::dsl::*;

    let results = document_revisions
        .filter(document_id.eq(doc_id))
        .load::<DocumentRevision>(conn)?;

    Ok(results)
}
