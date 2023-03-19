use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

use shared::models::tasks::*;

use crate::services::tasks::{update_task, delete_task};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub task: TaskRepresentation,
    pub callback_updated: Callback<TaskRepresentation>,
    pub callback_deleted: Callback<Uuid>,
}

#[styled_component(Task)]
pub fn task(props: &Props) -> Html {
    let edit_mode = use_state(|| false);
    let submitted = use_state(|| false);
    let update_info = use_state(|| props.task.clone());
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
                group_id: props.task.group_id.clone(),
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
                        props.callback_updated.emit(task);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let onclick_delete = {
        let props = props.clone();
        let submitted = submitted.clone();
        let error = error.clone();
        let update_info = update_info.clone();
        Callback::from(move |event: MouseEvent| {
            let props = props.clone();
            let submitted = submitted.clone();
            let error = error.clone();
            let update_info = update_info.clone();
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = delete_task(update_info.task_id.clone()).await;
                match result {
                    Ok(response) => {
                        submitted.set(false);
                        props.callback_deleted.emit(update_info.task_id);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let onclick_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |event: MouseEvent| {
            edit_mode.set(true);
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

    let onclick_save = {
        let props = props.clone();
        let edit_mode = edit_mode.clone();
        let submitted = submitted.clone();
        let error = error.clone();
        let update_info = update_info.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let props = props.clone();
            let edit_mode = edit_mode.clone();
            let submitted = submitted.clone();
            let error = error.clone();
            let update_info = update_info.clone();
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = update_task(
                    update_info.task_id.clone(),
                    TaskUpdatePayload {
                        group_id: props.task.group_id.clone(),
                        label: Some(update_info.label.clone()),
                        is_done: None,
                        order_index: None,
                    },
                )
                .await;
                match result {
                    Ok(task) => {
                        submitted.set(false);
                        edit_mode.set(false);
                        props.callback_updated.emit(task);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let onclick_cancel = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |event: MouseEvent| {
            let edit_mode = edit_mode.clone();
            edit_mode.set(false);
        })
    };


    html! {
        { if *edit_mode {
            html! {
                <div class="task">
                    <form onsubmit={onclick_save}>
                        <input type="checkbox" checked={*is_done} onclick={onclick_checkbox} disabled={*submitted}/>
                        <input type="text" value={update_info.label.clone()} oninput={oninput_label} disabled={*submitted}/>
                        <button class="page-btn" type="submit">{"✔"}</button>
                        <button class="page-btn" onclick={onclick_cancel}>{"✘"}</button>
                    </form>
                </div>
            }
        } else {
            html! {
                <div class="task">
                    <input type="checkbox" checked={*is_done} onclick={onclick_checkbox} disabled={*submitted}/>
                    <span>{props.task.label.clone()}</span>
                    <button class="edit-icon page-btn" onclick={onclick_edit}>{"✎"}</button>
                    <button class="page-btn" onclick={onclick_delete}>{"✘"}</button>
                </div>
            }
        }}
    }
}
