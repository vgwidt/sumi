use stylist::yew::styled_component;
use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen::JsCast;

use shared::models::tasks::*;

use crate::{services::tasks::{update_taskgroup, delete_taskgroup, get_group_tasks}, routes::ticket::{task::Task, task_new::NewTask}};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub taskgroup: TaskGroupRepresentation,
    pub callback_group_updated: Callback<TaskGroupRepresentation>,
    pub callback_group_deleted: Callback<Uuid>,
    pub callback_added: Callback<TaskGroupRepresentation>,
}

#[styled_component(TaskGroup)]
pub fn task_group(props: &Props) -> Html {
    let edit_mode = use_state(|| false);
    let submitted = use_state(|| false);
    let update_info = use_state(|| props.taskgroup.clone().label);
    let error = use_state(|| String::new());
    let new_task = use_state(|| TaskNewPayload {
        group_id: Uuid::new_v4(), 
        label: String::new(),
        is_done: false,
        order_index: 0,   
    });
    let adding_task = use_state(|| false);

    let onclick_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(true);
        })
    };

    let oninput_label = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            update_info.set(input.value());
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
            let updated_taskgroup = TaskGroupUpdatePayload {
                label: Some(update_info.to_string()),
                order_index: None,
            };
            wasm_bindgen_futures::spawn_local(async move {
                submitted.set(true);
                let result = update_taskgroup(props.taskgroup.clone().group_id, updated_taskgroup).await;
                match result {
                    Ok(taskgroup) => {
                        submitted.set(false);
                        edit_mode.set(false);
                        props.callback_group_updated.emit(taskgroup);
                    }
                    Err(e) => {
                        submitted.set(false);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let onclick_add_task = {
        let new_task = new_task.clone();
        let props = props.clone();
        let adding_task = adding_task.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlButtonElement>().value();
            web_sys::console::log_1(&value.clone().into());
            let group_uuid: Uuid = value.parse().unwrap();
            let task = TaskNewPayload {
                group_id: group_uuid,
                label: "New Task".to_string(),
                is_done: false,
                order_index: props.taskgroup.clone().tasks
                .clone()
                .iter()
                .map(|g| g.order_index)
                .max()
                .unwrap_or(0)
                + 1,
            };
            new_task.set(task);
            adding_task.set(true);
        })
    };


    let onclick_delete_group = {
        let props = props.clone();
        Callback::from(move |event: MouseEvent| {
            let props = props.clone();
            let target = event.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlButtonElement>().value();
            let group_uuid = value.parse().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                let result = delete_taskgroup(&group_uuid).await;
                match result {
                    Ok(_) => {
                        props.callback_group_deleted.emit(group_uuid);
                    }
                    Err(e) => {
                        log::error!("Error deleting task group: {}", e);
                    }
                }
            });
        })
    };

    //callback_updated for when a task is updated, lazily just refetch list of tasks for the group using get_group_tasks
    let callback_updated = { 
        let props = props.clone();
        let adding_task = adding_task.clone();
        Callback::from(move |_| {
            let props = props.clone();
            let adding_task = adding_task.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = get_group_tasks(props.taskgroup.clone().group_id).await;
                match result {
                    Ok(new_tasks) => {
                        let new_group = TaskGroupRepresentation {
                            group_id: props.taskgroup.clone().group_id,
                            label: props.taskgroup.clone().label,
                            order_index: props.taskgroup.clone().order_index,
                            tasks: new_tasks,
                        };
                        adding_task.set(false);
                        props.callback_group_updated.emit(new_group);
                    }
                    Err(e) => {
                        log::error!("Error fetching tasks for group {}: {}", props.taskgroup.clone().group_id, e);
                    }
                }
            });
        })
    };

    //callback_deleted for when a task is deleted
    let callback_deleted = {
        let props = props.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let props = props.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = get_group_tasks(props.taskgroup.clone().group_id).await;
                match result {
                    Ok(new_tasks) => {
                        let new_group = TaskGroupRepresentation {
                            group_id: props.taskgroup.clone().group_id,
                            label: props.taskgroup.clone().label,
                            order_index: props.taskgroup.clone().order_index,
                            tasks: new_tasks,
                        };
                        props.callback_group_updated.emit(new_group);
                    }
                    Err(e) => {
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let callback_task_added = {
        let props = props.clone();
        let adding_task = adding_task.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let props = props.clone();
            let adding_task = adding_task.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = get_group_tasks(props.taskgroup.clone().group_id).await;
                match result {
                    Ok(new_tasks) => {
                        let new_group = TaskGroupRepresentation {
                            group_id: props.taskgroup.clone().group_id,
                            label: props.taskgroup.clone().label,
                            order_index: props.taskgroup.clone().order_index,
                            tasks: new_tasks,
                        };
                        adding_task.set(false);
                        props.callback_added.emit(new_group);
                    }
                    Err(e) => {
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let onclick_cancel = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            let edit_mode = edit_mode.clone();
            edit_mode.set(false);
        })
    };

    html! {
            <div>
        {if *edit_mode {
            html! {
                <form onsubmit={onclick_save.clone()}>
                    <input type="text" value={update_info.to_string()} oninput={oninput_label.clone()}/>
                    <button type="submit" class="page-btn">{"✔"}</button>
                    <button class="page-btn" onclick={onclick_cancel}>{"✘"}</button>
                </form>
            }
        } else {
            html! {
                <div>
                <span style="font-weight: bold;">{&props.taskgroup.label}</span>
                <button name="new-task" class="add-icon page-btn" value={props.taskgroup.group_id.to_string()} onclick={onclick_add_task.clone()}>{"+"}</button>
                <button class="edit-icon page-btn" onclick={onclick_edit}>{"✎"}</button>
                <button class="page-btn" name="delete-group" value={props.taskgroup.group_id.to_string()} onclick={onclick_delete_group.clone()}>{"✘"}</button>
                </div>
            }
        }}
        <div>
            {for props.taskgroup.tasks.iter().map(|task| html! {
                <Task ticket_id={props.ticket_id} task={task.clone()} callback_updated={callback_updated.clone()} callback_deleted={callback_deleted.clone()}/>
            })}
            {if *adding_task {
                html! {
                    <NewTask ticket_id={props.ticket_id} group_id={props.taskgroup.group_id} task={TaskNewPayload {
                        group_id: new_task.group_id,
                        label: new_task.label.clone(),
                        is_done: new_task.is_done,
                        order_index: new_task.order_index,
                    }} callback_added={callback_task_added.clone()}/>
                }
            } else {
                html! {}
            }}
        </div>
    </div>
}
}