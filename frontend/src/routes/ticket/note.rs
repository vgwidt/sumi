use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::delete::{DeleteItem, ItemTypes};
use crate::components::time_format::TimeFormat;
use crate::hooks::use_language_context;
use crate::services::notes::update_note;
use crate::types::{NoteCreateInfo, NoteInfo};
use crate::utils::markdown_to_html;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub note: NoteInfo,
    pub callback: Callback<String>,
    pub callback_updated: Callback<NoteInfo>,
}

#[styled_component(Note)]
pub fn note(props: &Props) -> Html {
    let note = &props.note;
    let language = use_language_context();
    let edit_mode = use_state(|| false);
    let submitted = use_state(|| false);
    let update_info = use_state(|| note.clone());
    let error = use_state(|| String::new());

    //when submitted, send the note to the backend
    {
        let update_info = update_info.clone();
        let submitted = submitted.clone();
        let edit_mode = edit_mode.clone();
        let error = error.clone();
        let note_owner = note.owner.as_ref().unwrap().user_id;
        let callback_updated = props.callback_updated.clone();
        use_effect_with(submitted.clone(),move |submitted| {
            if **submitted {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = {
                        let request = NoteCreateInfo {
                            ticket: update_info.ticket,
                            owner: Some(note_owner),
                            text: update_info.text.clone(),
                            time: update_info.time,
                        };
                        update_note(update_info.note_id, request).await
                    };
        
                    match result {
                        Ok(note) => {
                            edit_mode.set(false);
                            callback_updated.emit(note);
                        }
                        Err(e) => {
                            error.set(e.to_string());
                        }
                    }
                });
                submitted.set(false);
            }
            || ()
        });
    }

    let on_submit = {
        let submitted = submitted.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(true);
        })
    };

    let on_click_cancel = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(false);
        })
    };

    // When editing
    let oninput_content = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.text = input.value();
            update_info.set(info);
        })
    };

    let onclick_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(true);
        })
    };

    html! {
        <div class="note">
            <div class="note-header">
                <span class="note-owner">
                    { note.display_name() }
                    {" "}
                    <TimeFormat time={note.created_at} />
                </span>
                <span>
                    <button class="btn" onclick={onclick_edit}>
                        {language.get("Edit")}
                    </button>
                    <DeleteItem
                        item_id={note.note_id.to_string()}
                        item_type={ItemTypes::Note}
                        callback={props.callback.clone()}
                    />
                </span>
            </div>
            { if *edit_mode { //if edit mode is true, show the edit form
                html! {
                    <div class="note-edit">
                        <form onsubmit={on_submit}>
                        <textarea placeholder="Text (Markdown)" rows=4 value={update_info.text.clone()} oninput={oninput_content} />
                            <div>
                            <button class="btn" type="submit">
                                { language.get("Submit") }
                            </button>
                            <button class="btn" onclick={on_click_cancel}>
                                {language.get("Cancel")}
                            </button>
                            </div>
                        </form>
                    </div>
                }
            } else {
                html! {
                    <div class="note-text">
                    { markdown_to_html(&note.text) }
                   </div>
                }
            }
        }
            //if time is not null or greater than 0, show the time spent
            { if note.time > 0 {
                html! {
                    <div class="note-time">
                        { format!("Time spent: {} minutes", note.time) }
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
