use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::delete::{DeleteItem, ItemTypes};
use crate::components::time_format::TimeFormat;
use crate::types::NoteInfo;
use crate::utils::markdown_to_html;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub note: NoteInfo,
    pub callback: Callback<String>,
}

#[styled_component(Note)]
pub fn note(props: &Props) -> Html {
    let note = &props.note;

    html! {
        <div class="note">
            <div class="note-header">
                <span class="note-owner">
                    { note.display_name() }
                    {" "}
                    <TimeFormat time={note.created_at} />
                </span>
                <DeleteItem
                    item_id={note.note_id.to_string()}
                    item_type={ItemTypes::Note}
                    callback={props.callback.clone()}
                />
            </div>
            <div class="note-text">
                { markdown_to_html(&note.text) }
            </div>
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
