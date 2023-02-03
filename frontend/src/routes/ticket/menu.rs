use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::delete::DeleteItem;
use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::routes::AppRoute;
use crate::services::tickets::update_status;
use crate::types::TicketStatusInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub ticket_status: String,
}

//Needs to be refactored to use new delete component
#[styled_component(TicketMenu)]
pub fn ticket_menu(props: &Props) -> Html {
    let theme = use_theme();
    let navigator = use_navigator().unwrap();
    let dropdown = use_state(|| false);
    let language = use_language_context();

    let callback_deleted = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Home);
        })
    };
    // use display: none; if not using state
    let style = style!(
        r#"
        .dropdown {
            position: relative;
            float: right;
          }
        .dropdown-content .btn {
            min-width: 100px;
          }
        .btn-action {
            font-size: 16px;
            min-width: 100px;
          }
        .btn-action:hover {
            border: 1px solid ${border};
        }
          "#,
        border = theme.border.clone(),
    )
    .expect("Failed to parse style");

    let onclick_edit = {
        let ticket_id = props.ticket_id.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Editor { ticket_id });
        })
    };

    let onclick_dropdown = {
        let dropdown = dropdown.clone();
        Callback::from(move |_| {
            dropdown.set(!*dropdown);
        })
    };

    let onclick_toggle_status = {
        let ticket_id = props.ticket_id.clone();
        let ticket_status = props.ticket_status.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let ticket = TicketStatusInfo {
                status: if ticket_status == "Closed" {
                    "Open".to_string()
                } else {
                    "Closed".to_string()
                },
            };
            //async block
            wasm_bindgen_futures::spawn_local(async move {
                let res = update_status(ticket_id, &ticket).await;
                match res {
                    Ok(_) => {
                        //navigator.push(&AppRoute::Ticket { ticket_id });
                    }
                    Err(e) => {
                        log::error!("Error updating ticket status: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <span class={style}>
            <div class="dropdown">
                <button class="btn-action" onclick={onclick_dropdown}>
                    { language.get("Actions") }
                </button>
                { if *dropdown { html! {
                <div class="dropdown-content">
                    <div>
                        <button class="btn" onclick={onclick_edit}>
                        { language.get("Edit") }
                        </button>
                    <div>
                        <button class="btn" onclick={onclick_toggle_status}>
                            //if ticket_status is open, show close ticket, else show open ticket
                            { if props.ticket_status != "Closed" { "Close Ticket" } else { "Open Ticket" } }
                        </button>
                    </div>
                    </div>
                    <div>
                    <DeleteItem
                        item_id={props.ticket_id.to_string()}
                        item_type={crate::components::delete::ItemTypes::Ticket}
                        callback={callback_deleted}
                    />
                    </div>
                </div>
                } } else { html! {} } }
            </div>
        </span>

    }
}
