use shared::models::{response::Response, tickets::TicketFilterPayload};

use super::{request_delete, request_get, request_post, request_put};
use crate::types::*;

//Get all tickets
// pub async fn all() -> Result<TicketListInfo, Error> {
//     let mut tickets: TicketListInfo = request_get::<TicketListInfo>(format!("/tickets")).await?;

//     tickets
//         .tickets
//         .sort_by(|a, b| a.ticket_id.cmp(&b.ticket_id));

//     Ok(tickets)
// }

// //Get tickets for a specific user
// pub async fn by_author(author: String) -> Result<TicketListInfo, Error> {
//     let mut tickets: TicketListInfo =
//         request_get::<TicketListInfo>(format!("/tickets/assignee/{}", author)).await?;

//     tickets
//         .tickets
//         .sort_by(|a, b| a.ticket_id.cmp(&b.ticket_id));

//     Ok(tickets)
// }

//get request that accepts optional status and assignee parameters
pub async fn get_filtered(
    query: &TicketFilterPayload,
) -> Result<TicketListInfo, Error> {

    let mut params = String::new();
    if let Some(status) = &query.status {
        params.push_str(&format!("status={}", status));
    }
    if let Some(assignee) = query.assignee {
        if params.len() > 0 {
            params.push_str("&");
        }
        params.push_str(&format!("assignee={}", assignee));
    }
    if let Some(page) = query.page {
        if params.len() > 0 {
            params.push_str("&");
        }
        params.push_str(&format!("page={}", page));
    }
    if let Some(per_page) = query.per_page {
        if params.len() > 0 {
            params.push_str("&");
        }
        params.push_str(&format!("per_page={}", per_page));
    }
    if let Some(sort_by) = &query.sort_by {
        if params.len() > 0 {
            params.push_str("&");
        }
        params.push_str(&format!("sort_by={}", sort_by));
    }
    if let Some(sort_order) = &query.sort_order {
        if params.len() > 0 {
            params.push_str("&");
        }
        params.push_str(&format!("sort_order={}", sort_order));
    }

    let tickets: TicketListInfo = request_get::<TicketListInfo>(format!("/tickets?{}", params)).await?;

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
pub async fn update_status(ticket_id: i32, status: &TicketStatusInfo) -> Result<Response<TicketInfo>, Error> {
    request_put::<&TicketStatusInfo, Response<TicketInfo>>(format!("/tickets/{}", ticket_id), status).await
}

pub async fn get_events(ticket_id: i32) -> Result<Vec<TicketEvent>, Error> {
    let mut events: Vec<TicketEvent> =
        request_get::<Vec<TicketEvent>>(format!("/tickets/{}/events", ticket_id)).await?;

    events.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    Ok(events)
}