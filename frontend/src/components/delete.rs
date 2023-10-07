use uuid::Uuid;
use yew::prelude::*;

use crate::components::confirmation::Confirmation;
use crate::hooks::use_language_context;
use crate::services::documents::delete_document;
use crate::services::notes::delete_note;
use crate::services::tickets::delete_ticket;

#[derive(Clone, PartialEq)]
pub enum ItemTypes {
    Document,
    Note,
    Ticket,
}

impl ItemTypes {
    pub fn to_string(&self) -> String {
        match self {
            ItemTypes::Document => "document",
            ItemTypes::Note => "note",
            ItemTypes::Ticket => "ticket",
        }
        .to_string()
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    //item id can be uuid or i32
    pub item_id: String,
    pub item_type: ItemTypes,
    pub callback: Callback<String>,
}

#[function_component(DeleteItem)]
pub fn delete_item(props: &Props) -> Html {
    let confirmation_pending = use_state(|| false);
    let delete_confirmation = use_state(|| false);
    let language = use_language_context();
    let deleted_item = use_state(|| None);

    {
        //if delete_confirmation changes to yes, run delete
        let deleted_item = deleted_item.clone();
        let delete_confirmation = delete_confirmation.clone();
        let props = props.clone();
        use_effect_with(delete_confirmation,move |delete_confirmation| {
            if **delete_confirmation {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = match props.item_type {
                        ItemTypes::Document => {
                            delete_document(Uuid::parse_str(&props.item_id).unwrap()).await
                        }
                        ItemTypes::Note => {
                            delete_note(Uuid::parse_str(&props.item_id).unwrap()).await
                        }
                        ItemTypes::Ticket => {
                            delete_ticket(props.item_id.parse::<i32>().unwrap()).await
                        }
                    };
                    if let Ok(_) = result {
                        deleted_item.set(Some(props.item_id));
                    }
                });
            }
            || ()
        })
    }

    let onclick = {
        let confirmation_pending = confirmation_pending.clone();
        Callback::from(move |_| {
            confirmation_pending.set(true);
        })
    };

    // let onclick_cancel = {
    //     let confirmation_pending = confirmation_pending.clone();
    //     Callback::from(move |_| {
    //         confirmation_pending.set(false);
    //     })
    // };

    {
        let callback = props.callback.clone();
        use_effect(move || {
            if let Some(item_id) = &*deleted_item {
                callback.emit(item_id.to_string());
            }
            || ()
        })
    }

    let message = format!(
        "{} {}",
        language.get("Are you sure you want to delete this"),
        props.item_type.to_string()
    );

    html! {
        <span>
            <button class="btn" onclick={onclick}>{language.get("Delete")}</button>
        { if *confirmation_pending {
            html! {
                <span>
                <Confirmation
                    message={message.clone()} callback={Callback::from(move |confirmed| {
                        if confirmed {
                            delete_confirmation.set(true);
                        }
                        confirmation_pending.set(false);
                    })} />
                    </span>
            }
        } else {
            html! {}
        }}
        </span>
    }
}
