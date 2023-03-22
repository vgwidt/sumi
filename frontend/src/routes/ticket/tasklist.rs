use stylist::yew::styled_component;
use yew::prelude::*;

use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::routes::ticket::taskgroup::TaskGroup;
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

    let onclick_add_group = {
        let tasklist = tasklist.clone();
        let props = props.clone();
        let error = error.clone();
        let language = language.clone();
        Callback::from(move |_| {
            let tasklist = tasklist.clone();
            let error = error.clone();
            let language = language.clone();
            tasklist.set(Tasklist {ticket_id: props.ticket_id, task_groups: vec![]}); //workaround to force rerender
            wasm_bindgen_futures::spawn_local(async move {
                let group_info = TaskGroupNewPayload {
                    label: format!("{} {}", language.get("Tasklist"), chrono::Local::now().format("%Y-%m-%d")),
                    //set order index to group with highest index + 1
                    order_index: tasklist
                        .task_groups
                        .clone()
                        .iter()
                        .map(|g| g.order_index)
                        .max()
                        .unwrap_or(0)
                        + 1,
                };
                let result = create_taskgroup(props.ticket_id, group_info).await;
                match result {
                    Ok(group) => {
                        //get new ticketlist
                        let new_tasks = get_tasklist(props.ticket_id).await.unwrap();
                        tasklist.set(new_tasks);
                    }
                    Err(e) => {
                        log::error!("Error creating task group: {}", e);
                        error.set(e.to_string());
                    }
                }
            });
        })
    };

    let callback_updated = {
        let tasklist = tasklist.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let tasklist = tasklist.clone();
            let props = props.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let new_tasks = get_tasklist(props.ticket_id).await.unwrap();
                tasklist.set(Tasklist { ticket_id: props.ticket_id, task_groups: new_tasks.task_groups });
            });
        })
    };

    let callback_deleted = {
        let tasklist = tasklist.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let tasklist = tasklist.clone();
            let props = props.clone();
            tasklist.set(Tasklist {ticket_id: props.ticket_id, task_groups: vec![]}); //workaround to force rerender
            wasm_bindgen_futures::spawn_local(async move {
                let updated_tasks = get_tasklist(props.ticket_id).await.unwrap();
                tasklist.set(updated_tasks);
            });
        })
    };

    let callback_added = {
        let tasklist = tasklist.clone();
        let props = props.clone();
        Callback::from(move |_| {
            let tasklist = tasklist.clone();
            let props = props.clone();
            tasklist.set(Tasklist {ticket_id: props.ticket_id, task_groups: vec![]}); //workaround to force rerender
            wasm_bindgen_futures::spawn_local(async move {
                let updated_tasks = get_tasklist(props.ticket_id).await.unwrap();
                tasklist.set(updated_tasks);
            });
        })
    };

    html! {
        <div>
            <div>
                <h3 class="section-header">
                    {language.get("Tasks")}
                    <button class="btn" onclick={onclick_add_group}>
                        {"New Tasklist"}
                    </button>
                </h3>
            </div>
            <div>
                {
                    for tasklist.task_groups.clone().into_iter().map(|group| {
                        html! {
                            <TaskGroup 
                                taskgroup={group.clone()}
                                ticket_id={props.ticket_id.clone()}
                                callback_group_updated={callback_updated.clone()}
                                callback_group_deleted={callback_deleted.clone()}
                                callback_added={callback_added.clone()}
                            />
                        }
                    })
                }
            </div>
        </div>
    }
}