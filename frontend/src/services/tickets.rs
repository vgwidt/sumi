use shared::models::response::Response;
use uuid::Uuid;

use super::{request_delete, request_get, request_post, request_put};
use crate::types::*;

//Get all tickets
pub async fn all() -> Result<TicketListInfo, Error> {
    let mut tickets: TicketListInfo = request_get::<TicketListInfo>(format!("/tickets")).await?;

    tickets
        .tickets
        .sort_by(|a, b| a.ticket_id.cmp(&b.ticket_id));

    Ok(tickets)
}

//Get tickets for a specific user
pub async fn by_author(author: String) -> Result<TicketListInfo, Error> {
    let mut tickets: TicketListInfo =
        request_get::<TicketListInfo>(format!("/tickets/assignee/{}", author)).await?;

    tickets
        .tickets
        .sort_by(|a, b| a.ticket_id.cmp(&b.ticket_id));

    Ok(tickets)
}

//get request that accepts optional status and assignee parameters
pub async fn get_filtered(
    assignee: Option<&Uuid>,
    status: Option<&String>,
    _priority: Option<&String>,
) -> Result<TicketListInfo, Error> {
    let mut tickets: TicketListInfo = {
        if let Some(status) = status {
            if let Some(assignee) = assignee {
                request_get::<TicketListInfo>(format!(
                    "/tickets?status={}&assignee={}",
                    status,
                    assignee.to_string()
                ))
                .await?
            } else {
                request_get::<TicketListInfo>(format!("/tickets?status={}", status)).await?
            }
        } else if let Some(assignee) = assignee {
            request_get::<TicketListInfo>(format!("/tickets?assignee={}", assignee)).await?
        } else {
            request_get::<TicketListInfo>(format!("/tickets")).await?
        }
    };

    tickets
        .tickets
        .sort_by(|a, b| a.ticket_id.cmp(&b.ticket_id));

    Ok(tickets)
}

pub async fn by_priority(priority: String) -> Result<TicketListInfo, Error> {
    request_get::<TicketListInfo>(format!("/tickets?priority={}", priority)).await
}

pub async fn delete_ticket(ticket_id: i32) -> Result<SuccessResponse, Error> {
    request_delete::<SuccessResponse>(format!("/tickets/{}", ticket_id)).await
}

pub async fn get(ticket_id: i32) -> Result<TicketInfo, Error> {
    let ticket: TicketInfo = request_get::<TicketInfo>(format!("/tickets/{}", ticket_id)).await?;

    Ok(ticket)
}

pub async fn update(
    ticket_id: i32,
    ticket: &TicketUpdateInfo,
) -> Result<Response<TicketInfo>, Error> {
    request_put::<&TicketUpdateInfo, Response<TicketInfo>>(
        format!("/tickets/{}", ticket_id),
        ticket,
    )
    .await
}

pub async fn create(ticket: &TicketCreateInfo) -> Result<Response<TicketInfo>, Error> {
    request_post::<&TicketCreateInfo, Response<TicketInfo>>("/tickets".to_string(), ticket).await
}

//Get notes for a ticket by ticket_id
pub async fn get_notes(ticket_id: i32) -> Result<Vec<NoteInfo>, Error> {
    let mut notes: Vec<NoteInfo> =
        request_get::<Vec<NoteInfo>>(format!("/tickets/{}/notes", ticket_id)).await?;

    //sort by created_at, temporary
    notes.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(notes)
}

//update status of a ticket
pub async fn update_status(ticket_id: i32, status: &TicketStatusInfo) -> Result<TicketInfo, Error> {
    request_put::<&TicketStatusInfo, TicketInfo>(format!("/tickets/{}", ticket_id), status).await
}
