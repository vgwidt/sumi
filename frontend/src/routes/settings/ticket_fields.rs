use web_sys::HtmlInputElement;
use yew::{prelude::*, suspense::use_future};

use crate::{hooks::{use_language_context, use_user_context}, types::{TicketCustomField, NewTicketCustomField}, contexts::language, services::tickets::{get_custom_fields, create_custom_field}};

#[function_component(TicketFields)]
pub fn ticket_fields() -> Html {
    let language = use_language_context();
    let error = use_state(|| String::new());
    let fields = use_future(|| async { get_custom_fields().await.unwrap_or_default() });

    let field_list = match fields {
        Ok(fields) => fields.clone(),
        Err(_) => vec![],
    };



    let onclick_add_field = {
        //will call call a new ticket field component with new flag
    };


    html! {
        <div>
            <h3 class="section-header">
                {language.get("Ticket Fields")}
            </h3>
            <button class="page-btn"> //onclick={onclick_add_field}>
                {language.get("Add Field")}
            </button>
            { for field_list.iter().map(|field| {
                html! {
                    <TicketField field={field.clone()} />
                }
            })}
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct TicketFieldProps {
    pub field: TicketCustomField,
    // pub callback: Callback<String>,
    // pub callback_updated: Callback<NoteInfo>,
}

//Component that actually displays a ticket_field, with a button to add
#[function_component(TicketField)]
pub fn ticket_field(props: &TicketFieldProps) -> Html {
    let language = use_language_context();

    html! {
        <div class="custom_field">
            <h3>{language.get("Field")}</h3>
        </div>
    }

}

#[derive(Properties, Clone, PartialEq)]
pub struct NewFieldProps {
    pub field: NewTicketCustomField,
    pub callback_added: Callback<TicketCustomField>,
}

#[function_component(NewTicketField)]
pub fn new_ticket_field(props: &NewFieldProps) -> Html {
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());
    let update_info = use_state(|| props.field.clone());

    let onclick_save = {
        let props = props.clone();
        let submitted = submitted.clone();
        let error = error.clone();
        let update_info = update_info.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let props = props.clone();
            let submitted = submitted.clone();
            let error = error.clone();
            let new_field = NewTicketCustomField {
                field_name: update_info.field_name.clone(),
                field_type: update_info.field_type.clone(),
                field_size: update_info.field_size.clone(),
                is_select: update_info.is_select.clone(),
                select_values: update_info.select_values.clone(),            
            };
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = create_custom_field(&new_field.clone()).await;
                match result {
                    Ok(response) => {
                        if response.success {
                            submitted.set(false);
                            props.callback_added.emit(response.data.unwrap());
                        } else {
                            submitted.set(false);
                            error.set(response.message.unwrap_or_else(|| "Unknown error".to_string()));
                        }
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    // let oninput_label = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         let mut info = (*update_info).clone();
    //         info.label = input.value();
    //         update_info.set(info);
    //     })
    // };

    // let oninput_check = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |e: InputEvent| {
    //         let input: HtmlInputElement = e.target_unchecked_into();
    //         let mut info = (*update_info).clone();
    //         info.is_done = input.checked();
    //         update_info.set(info);
    //     })
    // };
    
    html! {
        <div class="task">
            <form onsubmit={onclick_save}>
                // <input type="checkbox" checked={update_info.is_enabled.clone()} oninput={oninput_check} />
                // <input type="text" value={update_info.label.clone()} oninput={oninput_label}  placeholder="New field" />
                <button class="page-btn" type="submit">{"✔"}</button>
            </form>
        </div>
    }

}