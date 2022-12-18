use stylist::{style, yew::styled_component};
use uuid::Uuid;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::suspense::use_future;
use yew::suspense::use_future_with_deps;
use yew_router::prelude::Link;

use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;
use crate::services::{tickets::*, users::get_users};
use crate::types::TicketListInfo;

#[derive(Clone, Debug, PartialEq)]
pub struct Filter {
    pub assignee: Option<Uuid>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatusFilter {
    Open,
    Closed,
}

impl ToString for StatusFilter {
    fn to_string(&self) -> String {
        match self {
            StatusFilter::Open => "Open".to_string(),
            StatusFilter::Closed => "Closed".to_string(),
        }
    }
}

/// Ticket list sorting options, only partially implemented
#[derive(Clone, Debug, PartialEq)]
pub enum TicketListSort {
    ByTicketNoDesc,
    ByTicketNoAsc,
    ByUpdatedAtDesc,
    ByUpdatedAtAsc,
    ByAuthorDesc,
    ByAuthorAsc,
    ByPriorityDesc,
    ByPriorityAsc,
}

#[styled_component(TicketList)]
pub fn ticket_list() -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();
    let theme = use_theme();
    //let ticket_list = use_state(|| TicketListInfo::default());

    //For auto-updating the ticket list
    //let millis = use_state(|| 60000);

    let users = use_future(|| async { get_users().await.unwrap_or_default() });

    let userlist = match users {
        Ok(users) => users.clone(),
        Err(_) => vec![],
    };

    let filter = use_state(|| Filter {
        assignee: Some(user_ctx.user_id.clone()),
        status: Some(StatusFilter::Open.to_string()),
        priority: None,
    });
    //let sort = use_state(|| TicketListSort::ByTicketNoDesc);

    //API call to get (filtered) tickets
    let ticket_list = use_future_with_deps(
        |filter| async move {
            let filter = &*filter.clone();
            get_filtered(
                filter.assignee.as_ref(),
                filter.status.as_ref(),
                filter.priority.as_ref(),
            )
            .await
            .unwrap_or_default()
        },
        filter.clone(),
    );

    let ticket_list = match ticket_list {
        Ok(ticket_list) => ticket_list.clone(),
        Err(_) => TicketListInfo::default(),
    };

    let onclick_filter_assignee: Callback<Event> = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let mut new_filter = Filter {
                assignee: filter.assignee.clone(),
                status: filter.status.clone(),
                priority: filter.priority.clone(),
            };
            if value == "unassigned".to_string() {
                new_filter.assignee = Some(Uuid::nil());
            } else {
                new_filter.assignee = Some(Uuid::parse_str(&value).unwrap());
            }
            filter.set(new_filter);
        })
    };

    let onclick_filter_status = {
        let filter = filter.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            filter.set(Filter {
                assignee: filter.assignee.clone(),
                status: Some(value),
                priority: filter.priority.clone(),
            });
        })
    };

    let ticket_table_style = style!(
        r#"
        table.ticket-table {
            background-color: ${table_header};
        }
        th {
            background-color: ${table_header};
        }
        tr:nth-child(odd)
        {
            background-color: ${bg};
        }
        tr:nth-child(even)
        {
            background-color: ${bg};
        }
        tr:hover
        {
            background-color: ${table_header};
        }
        td {         
        }
        td a { 
            display: block; 
            padding: 10px 0px;
         }
         .ticket-table {
            margin-top: 12px;
            width: 100%; 
          }
          th:nth-child(1) {
            width: 7%;
          }
          th:nth-child(2) {
            width: 48%;
          }
          th:nth-child(3) {
            width: 10%;
          }
          th:nth-child(4) {
            width: 12%;
          }
          th:nth-child(5) {
            width: 12%;
          }
          th:nth-child(6) {
            width: 7%;
          }
          th:nth-child(7) {
            width: 4%;
          }
          tr {
            height: 32px;
          }
          td {
            padding-left: 2px;
          }
        "#,
        table_header = theme.table_header_color.clone(),
        bg = theme.background_color.clone(),
    )
    .expect("Failed to parse style");

    html! {
        <div style="margin: 2px 16px;">
            <div class="ticket-filters" style="display: flex; align-items: center;">
                <div>
                    <Link<AppRoute> to={AppRoute::EditorCreate} classes="btn">
                        { language.get("New Ticket") }
                    </Link<AppRoute>>
                </div>
                <div>
                    <label style="margin-left: 8px;" for="assignee">{"Assignee: "}</label>
                    <select name="assignee" id="assignee" onchange={onclick_filter_assignee}>
                        <option value={user_ctx.user_id.to_string()} selected=true>{user_ctx.display_name.clone()}
                        </option>
                        <option value="unassigned">{"Unassigned"}</option>
                        { for userlist.iter().map(|user| html! {
                        if user.user_id != user_ctx.user_id {
                        <option value={user.user_id.to_string()}>{user.display_name.clone()}</option>
                        }
                        })}
                    </select>
                    <label style="margin-left: 8px;" for="status">{"Status: "}</label>
                    <select name="status" id="status" onchange={onclick_filter_status}>
                        <option value="Open" selected=true>{language.get("Open")}</option>
                        <option value="Closed">{language.get("Closed")}</option>
                    </select>
                </div>
            </div>
            <div class={ticket_table_style}>
                <table class="table ticket-table">
                    <thead>
                        <tr>
                            <th scope="col">{language.get("Ticket No.")}</th>
                            <th scope="col">{language.get("Title")}</th>
                            <th scope="col">{language.get("Assignee")}</th>
                            <th scope="col">{language.get("Created")}</th>
                            <th scope="col">{language.get("Updated")}</th>
                            <th scope="col">{language.get("Priority")}</th>
                            <th scope="col"></th>
                        </tr>
                    </thead>
                    {
                    if !ticket_list.tickets.is_empty() {
                    html! {

                    <tbody>
                        {for ticket_list.tickets.iter().map(|ticket| {
                        html! {
                        <tr class="ticket-row">
                            <td>
                                <div>
                                    { &ticket.ticket_id }
                                </div>
                            </td>
                            <td>
                                <div>
                                    <Link<AppRoute>
                                        to={AppRoute::Ticket { ticket_id: ticket.ticket_id.clone() }}
                                        classes="preview-link" >
                                        { &ticket.title }
                                    </Link<AppRoute>>
                                </div>
                            </td>
                            <td>
                                <div class="info">
                                    { if let Some(assignee) = &ticket.assignee {
                                    &assignee.display_name
                                    } else {
                                    "Unknown"
                                    }}
                                </div>
                            </td>
                            <td>
                                <span class="date">
                                    { &ticket.created_at.format("%Y/%m/%d %H:%M") }
                                </span>
                            </td>
                            <td>
                                <span class="date">
                                    { &ticket.updated_at.format("%Y/%m/%d %H:%M") }
                                </span>
                            </td>
                            <td>
                                <span>
                                    { &ticket.priority }
                                </span>
                            </td>
                            <td>
                                <div class="edit-button">
                                    <Link<AppRoute>
                                        to={AppRoute::Editor {ticket_id: ticket.ticket_id.clone() }} >
                                        {language.get("Edit")}
                                    </Link<AppRoute>>
                                </div>
                            </td>
                        </tr>
                        }
                        })}
                    </tbody>

                    }
                    } else {
                        html! {
                            <tr>
                                <td colspan="7">{language.get("No tickets")}</td>
                            </tr>
                        }
                    }
                    }

                </table>
            </div>
        </div>
    }
}
