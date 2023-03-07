use gloo::utils::document;
use shared::models::tickets::TicketFilterPayload;
use stylist::{style, yew::styled_component};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew::suspense::use_future;
use yew::suspense::use_future_with_deps;
use yew_router::prelude::use_navigator;
use yew_router::prelude::Link;

use crate::components::loading::Loading;
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
    let navigator = use_navigator().unwrap();
    let ticket_list = use_state(|| TicketListInfo::default());
    let filter = use_state(|| TicketFilterPayload {
        assignee: Some(user_ctx.user_id.clone()),
        status: Some(StatusFilter::Open.to_string()),
        page: Some(1),
        per_page: Some(50),
        sort_by: Some("priority".to_string()),
        sort_order: Some("asc".to_string()),
        search: None,
    });
    let loading = use_state(|| false);

    let users = use_future(|| async { get_users().await.unwrap_or_default() });

    let userlist = match users {
        Ok(users) => users.clone(),
        Err(_) => vec![],
    };

    //API call to get (filtered) tickets
    {
        let filter = &*filter.clone();
        let loading = loading.clone();
        let ticket_list = ticket_list.clone();
        match use_future_with_deps(|filter| async move 
            { 
                let result = get_filtered(&filter).await;  
                match result {
                    Ok(tickets) => {
                        loading.set(false);
                        ticket_list.set(tickets);
                    },
                    Err(_) => {
                        loading.set(false);
                        ticket_list.set(TicketListInfo::default());
                    }
                }
            }, filter.clone()) {
                //results of future, I haven't mastered suspense yet
                Ok(_) => (),
                Err(_) => (),
            }
    }


    let onclick_filter_assignee: Callback<Event> = {
        let filter = filter.clone();
        let loading = loading.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            let mut new_filter = TicketFilterPayload {
                assignee: filter.assignee.clone(),
                status: filter.status.clone(),
                page: filter.page.clone(),
                per_page: filter.per_page.clone(),
                sort_by: filter.sort_by.clone(),
                sort_order: filter.sort_order.clone(),
                search: filter.search.clone(),
            };
            if value == "unassigned".to_string() {
                new_filter.assignee = Some(Uuid::nil());
            } else if value == "all".to_string() {
                new_filter.assignee = None;
            } else {
                new_filter.assignee = Some(Uuid::parse_str(&value).unwrap());
            }
            loading.set(true);
            filter.set(new_filter);
        })
    };

    let onclick_filter_status = {
        let filter = filter.clone();
        let loading = loading.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let value = input.value();
            loading.set(true);
            filter.set(TicketFilterPayload {
                assignee: filter.assignee.clone(),
                status: Some(value),
                page: filter.page.clone(),
                per_page: filter.per_page.clone(),
                sort_by: filter.sort_by.clone(),
                sort_order: filter.sort_order.clone(),
                search: filter.search.clone(),
            });
        })
    };

    let onclick_new = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::EditorCreate);
        })
    };

    let ticket_table_style = style!(
        r#"
        table.ticket-table {
            background-color: ${table_header};
        }
        th {
            background-color: ${table_header};
            cursor: pointer;
            user-select: none;
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
        table_header = theme.secondary_background.clone(),
        bg = theme.background.clone(),
    )
    .expect("Failed to parse style");

let onclick_prev_page = {
    let filter = filter.clone();
    let loading = loading.clone();
    Callback::from(move |_| {
        let mut new_filter = TicketFilterPayload {
            assignee: filter.assignee.clone(),
            status: filter.status.clone(),
            page: filter.page.clone(),
            per_page: filter.per_page.clone(),
            sort_by: filter.sort_by.clone(),
            sort_order: filter.sort_order.clone(),
            search: filter.search.clone(),
        };
        if new_filter.page.unwrap() > 1 {
            new_filter.page = Some(new_filter.page.unwrap() - 1);
        }
        loading.set(true);
        filter.set(new_filter);
    })
};

let onclick_next_page = {
    let filter = filter.clone();
    let loading = loading.clone();
    Callback::from(move |_| {
        let mut new_filter = TicketFilterPayload {
            assignee: filter.assignee.clone(),
            status: filter.status.clone(),
            page: filter.page.clone(),
            per_page: filter.per_page.clone(),
            sort_by: filter.sort_by.clone(),
            sort_order: filter.sort_order.clone(),
            search: filter.search.clone(),
        };
        new_filter.page = Some(new_filter.page.unwrap() + 1);
        loading.set(true);
        filter.set(new_filter);
    })
};

let onclick_filter_per_page = {
    let filter = filter.clone();
    let loading = loading.clone();
    Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let input: HtmlInputElement = document()
            .get_element_by_id("perpage")
            .unwrap()
            .unchecked_into();
        match input.value().parse::<i64>() {
            Ok(value) => {
                if value > 0 && value != filter.per_page.unwrap() {
                    loading.set(true);
                    filter.set(TicketFilterPayload {
                        assignee: filter.assignee.clone(),
                        status: filter.status.clone(),
                        page: Some(1),
                        per_page: Some(value),
                        sort_by: filter.sort_by.clone(),
                        sort_order: filter.sort_order.clone(),
                        search: filter.search.clone(),
                    });
                }
            }
            Err(_) => {}
        }
    })
};

    let onclick_search = {
        let filter = filter.clone();
        let loading = loading.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let input: HtmlInputElement = document().get_element_by_id("search").unwrap().unchecked_into();
            let value = input.value();
            //If Some(filter.search) matches value, do nothing.  If search is something, change it, if it is "", set it to None
            if filter.search.is_some() {
                if filter.search.as_ref().unwrap() != &value {
                    loading.set(true);
                    filter.set(TicketFilterPayload {
                        assignee: filter.assignee.clone(),
                        status: filter.status.clone(),
                        page: filter.page.clone(),
                        per_page: filter.per_page.clone(),
                        sort_by: filter.sort_by.clone(),
                        sort_order: filter.sort_order.clone(),
                        search: Some(value),
                    });
                }
            } else {
                if value != "" {
                    loading.set(true);
                    filter.set(TicketFilterPayload {
                        assignee: filter.assignee.clone(),
                        status: filter.status.clone(),
                        page: filter.page.clone(),
                        per_page: filter.per_page.clone(),
                        sort_by: filter.sort_by.clone(),
                        sort_order: filter.sort_order.clone(),
                        search: Some(value),
                    });
                }
            }
        })
    };

    let onclick_clear_filter = {
        let filter = filter.clone();
        let loading = loading.clone();
        Callback::from(move |_| {
            //only do if filter/search is not none
            if filter.search.is_some() {
                let input: HtmlInputElement = document().get_element_by_id("search").unwrap().unchecked_into();
                input.set_value("");
                loading.set(true);
                filter.set(TicketFilterPayload {
                    assignee: filter.assignee.clone(),
                    status: filter.status.clone(),
                    page: filter.page.clone(),
                    per_page: filter.per_page.clone(),
                    sort_by: filter.sort_by.clone(),
                    sort_order: filter.sort_order.clone(),
                    search: None,
                });
            }
        })
    };

    html! {
        <div style="margin: 2px 16px;">
            <div class="ticket-filters" style="display: flex; align-items: center;">
                <div>
                    <button class="btn" onclick={onclick_new}>
                        { language.get("New Ticket") }
                    </button>
                </div>
                <div>
                    <label style="margin-left: 8px;" for="assignee">{"Assignee: "}</label>
                    <select name="assignee" id="assignee" onchange={onclick_filter_assignee}>
                        <option value={user_ctx.user_id.to_string()} selected=true>{user_ctx.display_name.clone()}
                        </option>
                        <option value="all">{"(All)"}</option>
                        <option value="unassigned">{"(Unassigned)"}</option>
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
                <div>
                <form onsubmit={onclick_search} style="margin-left: 32px;">
                    <label for="search">{"Filter: "}</label>
                    <input style="margin: 0px;" type="text" id="search" placeholder={language.get("Filter")} />
                    <button class="page-btn" type="submit">
                        { language.get("✔") }
                    </button>
                    //ad button to clear filter, only show if filter is set
                    {if filter.search.is_some() {
                        html! {
                            <button class="page-btn" onclick={onclick_clear_filter}>
                                { "✘" }
                            </button>
                        }
                    } else {
                        html! {}
                    }}
               </form>
                </div>
            </div>
            <div class={ticket_table_style}>
                <table class="table ticket-table">
                    <thead>
                        <tr>
                            <th onclick={onclick_sort_by("ticket_id", &filter, &loading)} scope="col">{language.get("Ticket No.")}{if filter.sort_by.clone().unwrap() == "ticket_id" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
                            <th onclick={onclick_sort_by("title", &filter, &loading)} scope="col">{language.get("Title")}{if filter.sort_by.clone().unwrap() == "title" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
                            <th onclick={onclick_sort_by("assignee", &filter, &loading)} scope="col">{language.get("Assignee")}{if filter.sort_by.clone().unwrap() == "assignee" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
                            <th onclick={onclick_sort_by("created_at", &filter, &loading)} scope="col">{language.get("Created")}{if filter.sort_by.clone().unwrap() == "created_at" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
                            <th onclick={onclick_sort_by("updated_at", &filter, &loading)} scope="col">{language.get("Updated")}{if filter.sort_by.clone().unwrap() == "updated_at" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
                            <th onclick={onclick_sort_by("priority", &filter, &loading)} scope="col">{language.get("Priority")}{if filter.sort_by.clone().unwrap() == "priority" {if filter.sort_order.clone().unwrap() == "asc" {html! {"▼"}} else {html! {"▲"}}} else {html! {"　"}}}</th>
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
                                    &language.get("Unassigned")
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
                        }
                    }
                    }

                </table>
            </div>
            { 
                if *loading {
                html! {
                    <Loading />
                }
                } else if ticket_list.total_results == 0 {
                html!{ language.get("No results") }
                }
                else { 
                    html! {
                            <div class="pagination" style="display: flex; justify-content: space-between; margin-top: 6px;">
                                <div style="text-align: left;"> 
                                    <span>{format!("{} - {} of {}", ticket_list.page * filter.per_page.unwrap() - filter.per_page.unwrap() + 1, 
                                    if ticket_list.total_results > ticket_list.page * filter.per_page.unwrap() {
                                        ticket_list.page * filter.per_page.unwrap()
                                    } else {
                                        ticket_list.total_results
                                    }, 
                                    ticket_list.total_results)}</span>
                                </div>
                                <div style="text-align: center; flex: 1;">
                                <button class="page-btn" onclick={onclick_prev_page} disabled={ticket_list.page == 1}>
                                    {"❮"}
                                </button>
                                <span style="margin: 0px 8px;">{format!("{} / {}", ticket_list.page, ticket_list.total_pages)}</span>
                                <button class="page-btn" onclick={onclick_next_page} disabled={ticket_list.page == ticket_list.total_pages}>
                                    {"❯"}
                                </button>
                            </div>
                            </div>
                    }
                }
            }
            <div>
            <form style="display: inline;" onsubmit={onclick_filter_per_page}>
                <span>{language.get("Results per page: ")}</span>
                <input style="width: 64px; margin-right: 4px;" type="number" id="perpage" value={filter.per_page.unwrap().to_string()} />
                <button class="page-btn" type="submit">
                    {"✔️"}
                </button>
            </form>
            </div>
        </div>
    }
}

fn onclick_sort_by(sort_by: &str, filter: &UseStateHandle<TicketFilterPayload>, loading: &UseStateHandle<bool>) -> Callback<MouseEvent> {
    let filter = filter.clone();
    let sort_by = sort_by.to_string();
    let loading = loading.clone();
    Callback::from(move |_| {
        if !*loading {
        let mut new_filter = TicketFilterPayload {
            assignee: filter.assignee.clone(),
            status: filter.status.clone(),
            page: filter.page.clone(),
            per_page: filter.per_page.clone(),
            sort_by: Some(sort_by.clone()),
            sort_order: filter.sort_order.clone(),
            search: filter.search.clone(),
        };
        //if the new sort_by is different from the old filter.sort_by, then set the sort_order to "asc"
        if new_filter.sort_by.clone().unwrap_or_default() != filter.sort_by.clone().unwrap_or_default() {
            new_filter.sort_order = Some("asc".to_string());
        } else {
            if new_filter.sort_order.unwrap_or("asc".to_string()) == "asc" {
                new_filter.sort_order = Some("desc".to_string());
            } else {
                new_filter.sort_order = Some("asc".to_string());
            }
        }  
        loading.set(true);
        filter.set(new_filter);
    }

    })
}