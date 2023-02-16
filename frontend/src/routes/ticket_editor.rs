use shared::models::MAX_TITLE_LENGTH;
use stylist::style;
use stylist::yew::styled_component;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use web_sys::HtmlSelectElement;

use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

use crate::hooks::{use_language_context, use_user_context};
use crate::routes::AppRoute;
use crate::services::tickets::*;
use crate::services::users::get_users;
use crate::types::TicketCreateInfo;
use crate::types::TicketInfo;
use crate::types::TicketUpdateInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: Option<i32>,
}

//Create or update ticket
#[styled_component(TicketEditor)]
pub fn ticket_editor(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let loading = use_state(|| true);
    let language = use_language_context();
    let update_info = use_state(TicketCreateInfo::default);
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let retrieved_ticket = use_state(TicketInfo::default);

    let userlist = match { use_future(|| async { get_users().await.unwrap_or_default() }) } {
        Ok(users) => users.clone(),
        Err(_) => vec![],
    };

    //If props.ticket_id is some, get ticket info from server (retrieved_ticket)
    {
        let loading = loading.clone();
        let error = error.clone();
        let props = props.clone();
        let update_info = update_info.clone();
        let retrieved_ticket = retrieved_ticket.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(ticket_id) = props.ticket_id {
                        let result = get(ticket_id).await;
                        match result {
                            Ok(ticket) => {
                                retrieved_ticket.set(ticket.clone());
                                update_info.set(TicketCreateInfo {
                                    title: ticket.title,
                                    description: ticket.description,
                                    assignee: if let Some(assignee) = ticket.assignee {
                                        Some(assignee.user_id)
                                    } else {
                                        None
                                    },
                                    contact: ticket.contact,
                                    priority: ticket.priority,
                                    status: ticket.status,
                                });
                            }
                            Err(e) => {
                                error.set(e.to_string());
                            }
                        }
                    }
                    loading.set(false);
                });
                || ()
            },
            props.ticket_id.clone(),
        )
    }

    {
        //when submitted set to true, send update/create request to server
        let update_info = update_info.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let submitted = submitted.clone();
        let props = props.clone();
        let retrieved_ticket = retrieved_ticket.clone();
        use_effect_with_deps(
            move |submitted| {
                if **submitted {
                    wasm_bindgen_futures::spawn_local(async move {
                        let result = if let Some(ticket_id) = props.ticket_id {
                            let request = TicketUpdateInfo {
                                title: Some(update_info.title.clone()),
                                description: Some(update_info.description.clone()),
                                assignee: match update_info.assignee {
                                    Some(assignee) => {
                                        if assignee == Uuid::nil() {
                                            Some("".to_string())
                                        } else {
                                            Some(assignee.to_string())
                                        }
                                    }
                                    //Workaround until we know if the field was actually changed and needs to be sent or not
                                    None => Some("".to_string()),
                                },
                                contact: Some(update_info.contact.clone()),
                                priority: Some(update_info.priority.clone()),
                                status: Some(update_info.status.clone()),
                                version: Some(retrieved_ticket.revision.clone()),
                            };
                            update(ticket_id, &request).await
                        } else {
                            create(&update_info).await
                        };
                        match result {
                            Ok(response) => {
                                if response.success {
                                    navigator.push(&AppRoute::Ticket {
                                        ticket_id: response.data.unwrap().ticket_id,
                                    });
                                } else {
                                    error.set(
                                        response.message.unwrap_or("Unknown error".to_string()),
                                    );
                                }
                            }
                            Err(e) => {
                                error.set(e.to_string());
                            }
                        }
                    });
                }
                || ()
            },
            submitted.clone(),
        );
    }

    let onsubmit = {
        let submitted = submitted.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            submitted.set(true);
        })
    };
    let oninput_title = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.title = input.value();
            update_info.set(info);
        })
    };
    let oninput_description = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.description = input.value();
            update_info.set(info);
        })
    };
    //assignee is determined by UUID but displayed as username
    let onselect_assignee: Callback<Event> = {
        let update_info = update_info.clone();
        Callback::from(move |e: Event| {
            let input: HtmlSelectElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            //if Uuid is nil, assignee is set to None
            if input.value() == Uuid::nil().to_string() {
                info.assignee = None;
            } else {
                info.assignee = input.value().parse().ok();
            }
            update_info.set(info);
        })
    };

    let onselect_priority = {
        let update_info = update_info.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.priority = input.value();
            update_info.set(info);
        })
    };
    let onselect_status = {
        let update_info = update_info.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.status = input.value();
            update_info.set(info);
        })
    };
    // let oninput_contact = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         let mut info = (*update_info).clone();
    //         info.contact = input.value();
    //         update_info.set(info);
    //     })
    // };

    let style = style!(
        r#"
        input {
            width: 80%;
        }
        textarea {
            width: 80%;
        }
        "#
    )
    .expect("Failed to parse style");

    if user_ctx.is_authenticated() {
        if *loading {
            html! {}
        } else {
            html! {
                <div class={style}>
                <div class="error">
                    {error.to_string()}
                </div>
                    <form {onsubmit}>
                        <fieldset class="editor-text">
                            <legend>{language.get("Title")}</legend>
                            <input class="title" type="text" placeholder="Ticket Title"
                                value={update_info.title.clone()} oninput={oninput_title} maxlength={MAX_TITLE_LENGTH.to_string()}/>
                        </fieldset>
                        <fieldset class="editor-text">
                            <legend>{language.get("Description")}</legend>
                            <textarea class="description" rows="8"
                                placeholder="Ticket Description (Try using Markdown or HTML)"
                                value={update_info.description.clone()} oninput={oninput_description}>
                                </textarea>
                        </fieldset>
                        <fieldset class="editor-select">
                            <legend>{language.get("Assignee")}</legend>
                            <select onchange={onselect_assignee}>
                                <option value={Uuid::nil().to_string()} selected={update_info.assignee.unwrap_or_default() == Uuid::nil()}>{"Unassigned"}</option>
                                {
                                    for userlist.iter().map(|user| {
                                        html! {
                                        <option value={user.user_id.to_string()} selected={
                                            update_info.assignee.unwrap_or_default() == user.user_id}>
                                            {user.display_name.clone()}</option>
                                        }
                                    })
                                }
                            </select>
                        </fieldset>
                        <fieldset class="editor-select">
                            <legend>{language.get("Priority")}</legend>
                            <select onchange={onselect_priority}>
                                <option value="" selected={update_info.priority == ""}></option>
                                <option value="High" selected={update_info.priority=="High" }>{"High"}</option>
                                <option value="Medium" selected={update_info.priority=="Medium" }>{"Medium"}</option>
                                <option value="Low" selected={update_info.priority=="Low" }>{"Low"}</option>
                            </select>
                        </fieldset>
                        <fieldset class="editor-select">
                            <legend>{language.get("Status")}</legend>
                            <select onchange={onselect_status}>
                                <option value="Closed" selected={update_info.status=="Closed" }>{"Closed"}</option>
                                <option value="In Progress" selected={update_info.status=="In Progress" }>{"In Progress"}
                                </option>
                                <option value="Open" selected={update_info.status=="Open" }>{"Open"}</option>
                            </select>
                        </fieldset>
                        <button class="btn" type="submit">// disabled={ticket_update.loading}>
                            { language.get("Save") }
                        </button>
                    </form>
                </div>
            }
        }
    } else {
        html! {
        <Redirect<AppRoute> to={AppRoute::Login} />
        }
    }
}
