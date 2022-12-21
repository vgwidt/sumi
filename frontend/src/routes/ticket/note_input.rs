use stylist::yew::{styled_component, use_style};
use web_sys::HtmlInputElement;

use yew::prelude::*;

use crate::hooks::{use_language_context, use_user_context};
use crate::services::notes::*;
use crate::types::{NoteCreateInfo, NoteInfo};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub callback: Callback<NoteInfo>,
}

#[styled_component(NoteInput)]
pub fn note_input(props: &Props) -> Html {
    let create_info = use_state(NoteCreateInfo::default);
    let user_ctx = use_user_context();
    let language = use_language_context();
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());

    //When submitted set to true, send create request
    //On success, reset create_info, submitted, and error
    {
        let create_info = create_info.clone();
        let error = error.clone();
        let ticket_id = props.ticket_id.clone();
        let callback = props.callback.clone();
        use_effect_with_deps(
            move |submitted| {
                if **submitted {
                    let create_info = create_info.clone();
                    let error = error.clone();
                    let submitted = submitted.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let request = NoteCreateInfo {
                            ticket: ticket_id.clone(),
                            text: create_info.text.clone(),
                            owner: Some(user_ctx.user_id),
                            time: create_info.time.clone(),
                        };
                        let result = create(request).await;
                        match result {
                            Ok(note) => {
                                create_info.set(NoteCreateInfo::default());
                                submitted.set(false);
                                callback.emit(note.clone());
                                error.set(String::new());
                            }
                            Err(e) => {
                                error.set(e.to_string());
                                submitted.set(false);
                            }
                        }
                    });
                }
                || ()
            },
            submitted.clone(),
        )
    }

    let onsubmit = {
        let submitted = submitted.clone();
        let create_info = create_info.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if create_info.text.is_empty() && create_info.time == 0 {
                return;
            }
            submitted.set(true);
        })
    };
    let oninput = {
        let create_info = create_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*create_info).clone();
            info.text = input.value();
            create_info.set(info);
        })
    };
    let oninput_time = {
        let create_info = create_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*create_info).clone();
            if input.value().is_empty() {
                info.time = 0;
            } else {
                info.time = input.value().parse().unwrap();
            }
            create_info.set(info);
        })
    };

    let style = use_style! {
        r#"
        fieldset {
            padding-left: 0px;
            margin-left: 0px;
        }
        "#
    };

    html! {
        <form class={style} onsubmit={onsubmit}>
            <fieldset>
                <legend>{language.get("Add a note")}</legend>
                <div>
                    <textarea placeholder="Note Description (Markdown)" style="width: 95%;" rows="5" value={create_info.text.clone()}
                        oninput={oninput}>
                </textarea>
                </div>
                <div>
                    <label for="time">{"Time spent (minutes)"}</label>
                    <input type="number" min="0" step="5" style="width: 60px;"
                        value={create_info.time.clone().to_string()} oninput={oninput_time} />
                </div>
                <button class="btn" type="submit" disabled={*submitted}>
                    { "Post Note" }
                </button>
                <span class="error">
                    {error.to_string()}
                </span>
            </fieldset>
        </form>
    }
}
