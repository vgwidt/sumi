use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::delete::DeleteItem;
use crate::routes::AppRoute;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub ticket_status: String,
}

//Needs to be refactored to use new delete component
#[styled_component(TicketMenu)]
pub fn ticket_menu(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let callback_deleted = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Home);
        })
    };

    // let style = style!(
    //     r#"
    //     .dropdown {
    //         position: relative;
    //         float: right;
    //         padding: 12px 16px;
    //       }

    //       .dropdown-content {
    //         display: none;
    //         position: absolute;
    //         min-width: 160px;
    //         padding: 12px 16px;
    //         z-index: 1;
    //       }
    //       .dropdown:hover .dropdown-content {
    //         display: block;
    //       }
    //       "#,
    // )
    // .expect("Failed to parse style");

    let style = style!(
        r#"
        .dropdown {
            position: relative;
            float: right;
            display: inline-block;
        }
        "#,
    )
    .expect("Failed to parse style");

    let onclick_edit = {
        let ticket_id = props.ticket_id.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&AppRoute::Editor { ticket_id });
        })
    };

    html! {
        <span class={style}>
            <div class="dropdown">
                // <span>
                // { "Actions" }
                // </span>
                //<div class="dropdown-content">
                <button class="btn" onclick={onclick_edit}>
                { "Edit Ticket" }
                </button>
                //     <button class="btn" onclick={onclick_toggle_status}>
                //         //if ticket_status is open, show close ticket, else show open ticket
                //         { if props.ticket_status != "Closed" { "Close Ticket" } else { "Open Ticket" } }
                //     </button>
                <DeleteItem
                    item_id={props.ticket_id.to_string()}
                    item_type={crate::components::delete::ItemTypes::Ticket}
                    callback={callback_deleted}
                />
            </div>
        </span>

    }
}
