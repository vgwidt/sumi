use shared::models::users::UserDisplay;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::time_format::TimeFormat;
use crate::contexts::time::use_time;
use crate::hooks::use_language_context;
use crate::types::events::TicketEvent;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub event: TicketEvent,
    pub userlist: Vec<UserDisplay>,
}

#[styled_component(EventCard)]
pub fn event(props: &Props) -> Html {
    let event = &props.event;
    let language = use_language_context();
    let time_ctx = use_time();

    let actor_display = get_actor_display_name(props.userlist.clone(), event);

    //format the event
    let event_string = match event.event_type.as_str() {
        "assigned" => {
            if event.event_data == "" {
                format!("{} {}", actor_display, language.get("unassigned ticket"))
            } else {
                let target_display =
                    get_target_display_name(props.userlist.clone(), event.event_data.clone());
                format!(
                    "{} {} {}",
                    actor_display,
                    language.get("assigned ticket to"),
                    target_display
                )
            }
        }
        "status_updated" => {
            format!(
                "{} {} {}",
                actor_display,
                language.get("updated ticket status to"),
                event.event_data
            )
        }
        "priority_updated" => {
            format!(
                "{} {} {}",
                actor_display,
                language.get("updated ticket priority to"),
                event.event_data
            )
        }
        "title_updated" => {
            format!(
                "{} {} {}",
                actor_display,
                language.get("updated ticket title to"),
                event.event_data
            )
        }
        "due_date_updated" => {
            if event.event_data == "" {
                format!("{} {}", actor_display, language.get("removed due date"))
            } else {
                format!(
                    "{} {} {}",
                    actor_display,
                    language.get("updated due date to"),
                    //Parse due date from str to NaiveDateTime, convert to local time, then format
                    time_ctx
                        .convert_to_local(
                            &chrono::NaiveDateTime::parse_from_str(
                                &event.event_data,
                                "%Y-%m-%d %H:%M:%S"
                            )
                            .unwrap()
                        )
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                )
            }
        }
        _ => "Unknown event".to_string(),
    };

    html! {
        <div class="event-card">
            <TimeFormat time={event.created_at.clone()}/>
            {format!(": {}", event_string)}
        </div>
    }
}

fn get_actor_display_name(userlist: Vec<UserDisplay>, event: &TicketEvent) -> String {
    //match the user_id of event to a user_id in userlist
    let actor_display = match event.user_id {
        Some(uuid) => match userlist.iter().find(|user| user.user_id == uuid) {
            Some(user) => user.display_name.clone(),
            None => "unknown".to_string(),
        },
        None => "unknown".to_string(),
    };

    actor_display
}

//function to try to get display name by UUID
fn get_target_display_name(userlist: Vec<UserDisplay>, event_data: String) -> String {
    //first try to parse the Uuid, if it fails, return unknown, if it is "unassigned", return "unassigned"
    let target_display = match event_data.parse::<uuid::Uuid>() {
        Ok(uuid) => match userlist.iter().find(|user| user.user_id == uuid) {
            Some(user) => user.display_name.clone(),
            None => "unknown".to_string(),
        },
        Err(_) => {
            if event_data == "unassigned" {
                "unassigned".to_string()
            } else {
                "unknown".to_string()
            }
        }
    };

    target_display
}
