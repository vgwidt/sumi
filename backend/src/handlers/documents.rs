use super::super::DbPool;

use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{documents::*, Response};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/documents")]
async fn create(
    pool: web::Data<DbPool>,
    payload: web::Json<DocumentPayload>,
) -> Result<HttpResponse, Error> {
    let document = web::block(move || {
        let mut conn = pool.get()?;
        create_document(payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(document))
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
    payload: web::Json<DocumentPayload>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    
    //If version is in the payload, it means check revision
    if let Some(version) = payload.version {
        //Get the latest revision
        {
            let pool = pool.clone();
            let document_id = document_id.clone();
            let document =  web::block(move || {
                let mut conn = pool.get()?;
                get_document_by_id(document_id, &mut conn)
            })
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;
            
            if version != document.updated_at {
                let response = Response {
                    success: false,
                    message: "Document is out of date".to_string(),
                };
                return Ok(HttpResponse::Ok().json(response));
            }
        }
    }

    let document = web::block(move || {
        let mut conn = pool.get()?;
        update_document(document_id.into_inner(), payload.into_inner(), &mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(document))
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
        let response = Response {
            success: true,
            message: "Document deleted".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    } else {
        let response = Response {
            success: false,
            message: "Document not found".to_string(),
        };
        Ok(HttpResponse::Ok().json(response))
    }
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

fn create_document(payload: DocumentPayload, conn: &mut PgConnection) -> Result<Document, DbError> {
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
    payload: DocumentPayload,
    conn: &mut PgConnection,
) -> Result<Document, DbError> {
    use crate::schema::documents::dsl::*;

    let mut adjusted_title = payload.title;
    if adjusted_title.is_empty() {
        adjusted_title = "Untitled".to_string();
    };

    let result = diesel::update(documents.find(id))
        .set((
            parent_id.eq(payload.parent_id),
            //url.eq(payload.url),
            title.eq(adjusted_title),
            content.eq(payload.content),
            updated_at.eq(chrono::Utc::now().naive_utc()),
            updated_by.eq(payload.updated_by),
            archived.eq(payload.archived),
        ))
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
