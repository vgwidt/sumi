use std::result;

use stylist::style;
use stylist::yew::styled_component;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::routes::ticket::task::Task;
use crate::routes::ticket::task_new::NewTask;
use crate::services::tasks::*;
use shared::models::tasks::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
}

#[styled_component(TaskList)]
pub fn task_list(props: &Props) -> Html {
    let theme = use_theme();
    let language = use_language_context();
    let error = use_state(|| String::new());
    let new_task = use_state(|| TaskNewPayload {
        group_id: Uuid::new_v4(), 
        label: String::new(),
        is_done: false,
        order_index: 0,   
    });

    let tasklist: UseStateHandle<Tasklist> = use_state(|| Tasklist {
        ticket_id: props.ticket_id.clone(),
        task_groups: vec![],
    });

    {
        let tasklist = tasklist.clone();
        let props = props.clone();
        use_effect_with_deps(
            move |_| {
                let tasklist = tasklist.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let tasks = get_tasklist(props.ticket_id).await.unwrap();
                    tasklist.set(tasks);
                });
                || ()
            },
            props.ticket_id.clone(),
        )
    }

    
    let onclick_add_task = {
        //If no group (button value is empty), create group first, then create task using that group ID
        let tasklist = tasklist.clone();
        let new_task = new_task.clone();
        let props = props.clone();
        let error = error.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<web_sys::HtmlButtonElement>().value();
            web_sys::console::log_1(&value.clone().into());
            let mut group_uuid = uuid::Uuid::new_v4();
            if value.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    let group_info = TaskGroupNewPayload {
                        label: "New Group".to_string(),
                        order_index: 0,
                    };
                    let group = create_taskgroup(props.ticket_id, group_info).await.unwrap();
                    group_uuid = group.group_id;
                    });
                }
            else {
                group_uuid = value.parse().unwrap();
            }
            let task = TaskNewPayload {
                group_id: group_uuid,
                label: "New Task".to_string(),
                is_done: false,
                order_index: 0,
            };
            new_task.set(task);
        })
    };

    let onclick_add_group = {
        let tasklist = tasklist.clone();
        let props = props.clone();
        let error = error.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let group_info = TaskGroupNewPayload {
                    label: "New Group".to_string(),
                    order_index: 0,
                };
                let result = create_taskgroup(props.ticket_id, group_info).await;
                match result {
                    Ok(group) => {
                        //tasklist.set(new_tasklist);
                    }
                    Err(e) => {
                        log::error!("Error creating task group: {}", e);
                        //error.set(e.to_string());
                    }
                }
            });
        })
    };

    html! {
        //Display lists of tasks under each task group which is displayed as header.
        <div>
            <div>
                <h2>{language.get("tasks")}</h2>
                //If tasklist is empty, display +Task button to add new task.
                {if tasklist.clone().task_groups.is_empty() {
                    html! {
                        <button name="new-task-no-groups" value="" onclick={onclick_add_task.clone()}>{"+Task"}</button>
                    }
                } else {
                    html! {}
                }}
                <button onclick={onclick_add_group}>{"+Group"}</button>
            </div>
            <div>
                {for tasklist.clone().task_groups.iter().map(|group| html! {
                    <div>
                        <h3>
                        {&group.label}
                        <button name="new-task" value={group.group_id.to_string()} onclick={onclick_add_task.clone()}>{"+Task"}</button>
                        </h3>
                        <div>
                            {for group.tasks.iter().map(|task| html! {
                                <>
                                <Task ticket_id={props.ticket_id} task={task.clone()} callback={Callback::noop()} callback_updated={Callback::noop()}/>
                                { if new_task.clone().group_id == group.group_id {
                                    let new_task = new_task.clone();
                                    html! {
                                        <NewTask ticket_id={props.ticket_id} task={TaskNewPayload {
                                            group_id: new_task.group_id,
                                            label: new_task.label.clone(),
                                            is_done: new_task.is_done,
                                            order_index: new_task.order_index,
                                        }} callback={Callback::noop()} callback_updated={Callback::noop()}/>
                                    }
                                } else {
                                    html! {}
                                }}
                                </>
                            })}
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}