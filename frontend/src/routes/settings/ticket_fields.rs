use web_sys::HtmlInputElement;
use yew::{prelude::*, suspense::use_future};

use crate::{hooks::{use_language_context, use_user_context}, types::{TicketCustomField, NewTicketCustomField}, contexts::language, services::tickets::{get_custom_fields, create_custom_field}};

#[function_component(TicketFields)]
pub fn ticket_fields() -> Html {
    let language = use_language_context();
    let error = use_state(|| String::new());
    let fields = use_future(|| async { get_custom_fields().await.unwrap_or_default() });
    let adding_field = use_state(|| false);

    let field_list = match fields {
        Ok(fields) => fields.clone(),
        Err(_) => vec![],
    };

    let onclick_add_field = {
        //set adding_field to true
        let adding_field = adding_field.clone();
        Callback::from(move |event: MouseEvent| {
            adding_field.set(true);
        })
    };

    let callback_field_added = {
        let error = error.clone();
        let adding_field = adding_field.clone();
        Callback::from(move |field: TicketCustomField| {
            adding_field.set(false);
        })
    };


    html! {
        <div>
            <h3 class="section-header">
                {language.get("Ticket Fields")}
            </h3>
            <button class="btn" onclick={onclick_add_field}>
                {language.get("Add Field")}
            </button>
            <table>
                <tr>
                    <th>{language.get("Field Name")}</th>
                    <th>{language.get("Field Type")}</th>
                    <th>{language.get("Field Size")}</th>
                    <th>{language.get("Select Values")}</th>
                    <th>{language.get("Order Index")}</th>
                </tr>
                { for field_list.iter().map(|field| {
                    html! {
                        <TicketField field={field.clone()} />
                    }
                })}
            </table>
            {if *adding_field {
                html! {
                    <NewTicketField field={NewTicketCustomField {
                        field_name: "".to_string(),
                        field_type: "".to_string(),
                        field_size: 0,
                        select_values: None,
                        order_index: 0,
                    }} callback_added={callback_field_added.clone()}/>
                }
            } else {
                html! {}
            }}
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
            <tr>
                <td>
                    {&props.field.field_name}
                </td>
                <td>
                    {&props.field.field_type}
                </td>
                <td>
                    {&props.field.field_size}
                </td>
                <td>
                { if let Some(select_values) = &props.field.select_values {
                    html! {
                        <ul>
                            { for select_values.iter().map(|value| {
                                html! {
                                    <li>{value.clone().unwrap_or_else(|| "".to_string())}</li>
                                }
                            })}
                        </ul>
                    }
                } else {
                    html! {}
                }}
                </td>
                <td>
                    {&props.field.order_index}
                </td>
            </tr>
    }

}

#[derive(Properties, Clone, PartialEq)]
pub struct NewFieldProps {
    pub field: NewTicketCustomField,
    pub callback_added: Callback<TicketCustomField>,
}

#[function_component(NewTicketField)]
pub fn new_ticket_field(props: &NewFieldProps) -> Html {
    let language = use_language_context();
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
                select_values: update_info.select_values.clone(),   
                order_index: update_info.order_index.clone(),         
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

    let oninput_name = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.field_name = input.value();
            update_info.set(info);
        })
    };

    let oninput_type = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.field_type = input.value();
            update_info.set(info);
        })
    };

    let oninput_size = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.field_size = input.value().parse::<i32>().unwrap();
            update_info.set(info);
        })
    };

    let oninput_select_values = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            //If empty, set to None. We'll prevent accepting values depending on field type later
            if input.value().is_empty() {
                info.select_values = None;
            } else {
                //Split by newlines
                let values = input.value().split("\n").map(|value| {
                    //If empty, set to None
                    if value.is_empty() {
                        None
                    } else {
                        Some(value.to_string())
                    }
                }).collect::<Vec<Option<String>>>();
                info.select_values = Some(values);
            }
            update_info.set(info);
        })
    };

    let oninput_order_index = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.order_index = input.value().parse::<i32>().unwrap();
            update_info.set(info);
        })
    };

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
        <div class="add-ticket-field">
            <form onsubmit={onclick_save}>
                //Ask for field type (select box), field name, field size, select values (if applicable), order index
                <label>
                    {language.get("Field Type")}
                    <select oninput={oninput_type}>
                        <option value="text">{language.get("Text")}</option>
                        <option value="number">{language.get("Number")}</option>
                        <option value="date">{language.get("Date")}</option>
                        <option value="select">{language.get("Select")}</option>
                    </select>
                </label>
                <label>
                    {language.get("Field Name")}
                    <input type="text" name="field_name" required=true oninput={oninput_name} />
                </label>
                <label>
                    {language.get("Field Size")}
                    <input type="number" name="field_size" required=true oninput={oninput_size} />
                </label>
                <label>
                    {language.get("Select Values")}
                    <textarea name="select_values" oninput={oninput_select_values} />
                </label>
                <label>
                    {language.get("Order Index")}
                    <input type="number" name="order_index" required=true oninput={oninput_order_index} />
                </label>
                <button class="page-btn" type="submit">{"✔"}</button>
            </form>
        </div>
    }

}