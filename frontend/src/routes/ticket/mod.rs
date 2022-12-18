mod menu;
mod note;
mod note_input;
mod note_list;

use stylist::style;
use stylist::yew::styled_component;

use yew::prelude::*;

use crate::services::tickets::*;
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

    let style = style!(
        r#"
        margin: 0 auto;
        width: 95%;
        .ticket-detail {
            margin-left: 16px;
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
        .note-list {
            margin-top: 16px;
        }
          "#,
    )
    .expect("Failed to parse style");

    //Default ticket id is 0, so we don't want to render anything until we have a valid ticket id
    //If we change this we need to fix our unwraps
    if ticket.ticket_id != 0 {
        html! {
            <div class={style}>
                <div class="ticket-detail">
                    <div class="header">
                            <span class="ticket-id">{"(#"}{&ticket.ticket_id}{") "}</span>
                            <span class="title">{&ticket.title}</span>
                            <span>
                            <TicketMenu ticket_id={props.ticket_id} ticket_status={ticket.status.clone()}/>
                            </span>
                    </div>
                    <div class="assignee">
                        { "Assigned to: " }
                        { ticket.assignee.as_ref().unwrap().display_name.clone() }
                    </div>
                    <div class="created-date">
                        { "Created " }
                        { ticket.created_at.format("%Y-%m-%d %H:%M").to_string() }
                    </div>
                    <div class="updated-date">
                    { "Updated " }
                    { ticket.created_at.format("%Y-%m-%d %H:%M").to_string() }
                    </div>
                    <div class="description">
                        { markdown_to_html(&ticket.description) }
                    </div>
                </div>
                <hr />
                <div class="note-list">
                    <NoteList ticket_id={props.ticket_id.clone()} />
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
