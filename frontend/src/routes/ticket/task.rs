use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

use shared::models::tasks::*;

use crate::services::tasks::update_task;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub task: TaskRepresentation,
    pub callback: Callback<String>,
    pub callback_updated: Callback<TaskRepresentation>,
}

#[styled_component(Task)]
pub fn task(props: &Props) -> Html {
    let task = &props.task;
    let edit_mode = use_state(|| false);
    let submitted = use_state(|| false);
    let update_info = use_state(|| task.clone());
    let error = use_state(|| String::new());
    let is_done = use_state(|| props.task.is_done);

    //When checkbox is clicked, update the task's is_done value and send it to the server
    let onclick_checkbox = {
        let props = props.clone();
        let is_done = is_done.clone();
        let submitted = submitted.clone();
        let error = error.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().checked();
            let props = props.clone();
            let submitted = submitted.clone();
            let error = error.clone();
            let is_done = is_done.clone();
            let updated_task = TaskUpdatePayload {
                label: None,
                is_done: Some(value),
                order_index: None,
            };
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = update_task(props.task.task_id, updated_task).await;
                match result {
                    Ok(task) => {
                        submitted.set(false);
                        is_done.set(value);
                        //props.callback_updated.emit(task);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="task">
            <input type="checkbox" checked={*is_done} onclick={onclick_checkbox} disabled={*submitted}/>
            <span>{props.task.label.clone()}</span>
        </div>
    }

}