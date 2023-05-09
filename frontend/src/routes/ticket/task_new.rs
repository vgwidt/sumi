use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use shared::models::tasks::*;

use crate::services::tasks::create_task;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub tasklist_id: Uuid,
    pub task: TaskNewPayload,
    pub callback_added: Callback<TaskRepresentation>,
}

#[styled_component(NewTask)]
pub fn new_task(props: &Props) -> Html {
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());
    let update_info = use_state(|| props.task.clone());

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
            let new_task = TaskNewPayload {
                label: update_info.label.clone(),
                is_done: update_info.is_done,
                ..props.task.clone()                
            };
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = create_task(props.tasklist_id, new_task.clone()).await;
                match result {
                    Ok(task) => {
                        submitted.set(false);
                        props.callback_added.emit(task);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let oninput_label = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.label = input.value();
            update_info.set(info);
        })
    };

    let oninput_check = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.is_done = input.checked();
            update_info.set(info);
        })
    };
    
    html! {
        <div class="task">
            <form onsubmit={onclick_save}>
                <input type="checkbox" checked={update_info.is_done.clone()} oninput={oninput_check} />
                <input type="text" value={update_info.label.clone()} oninput={oninput_label}  placeholder="New task" />
                <button class="page-btn" type="submit">{"✔"}</button>
            </form>
        </div>
    }

}