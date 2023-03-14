mod menu;
mod note;
mod note_input;
mod note_list;
mod event;
mod tasklist;

use stylist::style;
use stylist::yew::styled_component;

use yew::prelude::*;
use yew::suspense::use_future;

use crate::routes::ticket::tasklist::TaskList;
use crate::services::tickets::*;
use crate::services::users::get_display_names;
use crate::types::TicketInfo;
use crate::utils::markdown_to_html;
use menu::TicketMenu;
pub use note_list::NoteList;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
}

/// Ticket detail page
#[styled_component(Ticket)]
pub fn ticket(props: &Props) -> Html {
    let ticket = use_state(|| TicketInfo::default());
    {
        let ticket = ticket.clone();
        let props = props.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let ticket_data = get(props.ticket_id).await.unwrap();
                    ticket.set(ticket_data);
                });
                || ()
            },
            props.ticket_id.clone(),
        )
    }

    //Adding this for now just to pass to events, but it will also be needed for inline editing in the future
    let userlist = match { use_future(|| async { get_display_names().await.unwrap_or_default() }) } {
        Ok(users) => users.clone(),
        Err(_) => vec![],
    };

    let style = style!(
        r#"
        margin: 0 auto;
        width: 95%;
        .ticket-detail {
            margin-bottom: 16px;
        }
          .header{
            padding-bottom: 12px;
          }
          .ticket-id {
            color: #838383;
          }
          .title {
            font-size: 20px;
            font-weight: bold;
          }
          .assignee {
            color: #838383;
            font-style: italic;
          }
          .created-date {
            color: #838383;
            font-style: italic;
          }
          .updated-date {
            color: #838383;
            font-style: italic;
          }
        .description {
            word-wrap: break-word;
        }
        .note-list {
            margin-top: 16px;
        }
        .status-badge {
            margin-left: 8px;
            padding: 4px 6px;
            border-radius: 8px;
            font-size: 10px;
            font-weight: bold;
            text-transform: uppercase;
            border: 1px solid;
            position: absolute;
        }
        .status-Open {
            background-color:rgba(63, 223, 63, 0.5);
        }
        .status-Closed {
            background-color:rgba(255, 63, 63, 0.5);
        }
          "#,
    )
    .expect("Failed to parse style");

    let callback_updated = {
        //we receive the new TicketInfo from callback, so we can just set it
        let ticket = ticket.clone();
        Callback::from(move |new_ticket: TicketInfo| {
            ticket.set(new_ticket);
        })
    };

    //Default ticket id is 0, so we don't want to render anything until we have a valid ticket id
    //If we change this we need to fix our unwraps
    if ticket.ticket_id != 0 {
        html! {
            <div class={style}>
                <div class="ticket-detail">
                    <div class="header">
                        <span class="ticket-id">{"(#"}{&ticket.ticket_id}{") "}</span>
                        <span class="title">{&ticket.title}</span>
                        <span class={format!("status-badge status-{}", ticket.status.clone())}>{&ticket.status}</span>
                        <span>
                        <TicketMenu ticket_id={props.ticket_id} ticket_status={ticket.status.clone()} callback={callback_updated} />
                        </span>
                    </div>
                    <div class="assignee">
                        { "Assigned to: " }
                        { if ticket.assignee.is_some() {
                            html!  { ticket.assignee.as_ref().unwrap().display_name.clone() }
                        } else {
                            html! { "Unassigned" }
                        } }
                    </div>
                    <div class="created-date">
                        { "Created " }
                        { ticket.created_at.format("%Y-%m-%d %H:%M").to_string() }
                    </div>
                    <div class="updated-date">
                        { "Updated " }
                        { ticket.updated_at.format("%Y-%m-%d %H:%M").to_string() }
                    </div>
                    <div class="description">
                        { markdown_to_html(&ticket.description) }
                    </div>
                </div>
                <hr />
                <div class="note-list">
                    <NoteList ticket_id={props.ticket_id.clone()} userlist={userlist} />
                </div>
                //tasklist
                <div class="tasklist">
                    <TaskList ticket_id={props.ticket_id.clone()} />
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
