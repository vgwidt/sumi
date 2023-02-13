use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use super::note::Note;
use super::note_input::NoteInput;
use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::services::tickets::{get_notes, get_events};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
}

//List of notes used by ticket detail page
#[styled_component(NoteList)]
pub fn note_list(props: &Props) -> Html {
    let theme = use_theme();
    let language = use_language_context();

    let note_list = use_state(|| vec![]);
    let event_list = use_state(|| vec![]);

    {
        let note_list = note_list.clone();
        let props = props.clone();
        use_effect_with_deps(
            move |_| {
                let note_list = note_list.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let notes = get_notes(props.ticket_id).await.unwrap();
                    note_list.set(notes);
                });
                || ()
            },
            props.ticket_id.clone(),
        )
    }

    {
        let event_list = event_list.clone();
        let props = props.clone();
        use_effect_with_deps(
            move |_| {
                let event_list = event_list.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let events = get_events(props.ticket_id).await.unwrap();
                    event_list.set(events);
                });
                || ()
            },
            props.ticket_id.clone(),
        )
    }


    let callback_added = {
        let note_list = note_list.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let note_list = note_list.clone();
            let props = props.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let notes = get_notes(props.ticket_id).await.unwrap();
                note_list.set(notes);
            });
        })
    };

    let callback_deleted = {
        let note_list = note_list.clone();
        let props = props.clone();
        Callback::from(move |_| {
            //Issue: if we don't clear the vector before fetching the new note, it loops infinitely
            //This will just update the vector later using callback value rather than send new API request
            note_list.set(vec![]);
            let note_list = note_list.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let notes = get_notes(props.ticket_id).await.unwrap();
                note_list.set(notes);
            });
        })
    };

    //Simple workaround to update the note list when a note is updated
    //To-do: callback values will be used to update the note list without sending new API request
    let callback_updated = {
        let note_list = note_list.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let note_list = note_list.clone();
            let props = props.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let notes = get_notes(props.ticket_id).await.unwrap();
                note_list.set(notes);
            });
        })
    };

    let inputnode = html! {
    <div>
        <NoteInput
            ticket_id={props.ticket_id.clone()}
            callback={callback_added} />
     </div>
    };

    let liststyle = style! {
        r#"
        "#,
    }
    .expect("Failed to parse style");

    let notestyle = style!(
        r#"
        .note {
            display: flex;
            flex-direction: column;
            margin: 0.75rem 0;
            border-radius: 0.5rem;
            border: 1px solid ${border};
        }
        .timeformat {
            font-size: 0.8rem;
            color: #838383;
        }
        .note-header {
            border-bottom: 1px solid ${border};
            align-items: center;
            display: flex;
            justify-content: space-between;
            background: ${headerbg};
            padding: 0.2rem 0.75rem;
            border-top-left-radius: inherit;
            border-top-right-radius: inherit;
        }
        .note-text{
            padding: 0rem 0.75rem;
        }
        .note-time {
            padding: 0.5rem 0.75rem;
            font-style: italic;
            color: #838383;
        }
        "#,
        headerbg = theme.secondary_background.clone(),
        border = theme.border.clone(),
    )
    .expect("Failed to parse style");

    let listnode = if note_list.len() > 0 {
        let note_list = &*note_list.clone();
        let event_list = &*event_list.clone();
        html! {
                <div class={liststyle}>
                    <h3>{language.get("Notes")}</h3>
                    <div class={notestyle}>
                        {for note_list.into_iter().map(|note| {
                        html! {
                        <Note ticket_id={props.ticket_id.clone()} note={note.clone()}
                            callback={callback_deleted.clone()} callback_updated={callback_updated.clone()} />
                        }
                        })}
                    </div>
                    <h3>{language.get("Events")}</h3>
                    <div class="event-list">
                        {for event_list.into_iter().map(|event| {
                        html! {
                            <p>{format!("{}: {}", event.created_at, event.event_type)}</p>
                        }
                        })}
                    </div>
                </div>
        }
    } else {
        html! {}
    };

    html! {
        <div>
            { inputnode }
            { listnode }
        </div>
    }
}
