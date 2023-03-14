use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
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

    html! {
        //Display lists of tasks under each task group which is displayed as header.
        <div>
            <div>
                <h2>{language.get("tasks")}</h2>
            </div>
            <div>
                {for tasklist.clone().task_groups.iter().map(|group| html! {
                    <div>
                        <h3>{&group.label}</h3>
                        <div>
                            {for group.tasks.iter().map(|task| html! {
                                <div>
                                    <input type="checkbox" checked={task.is_done} />
                                    <span>{&task.label}</span>
                                </div>
                            })}
                        </div>
                    </div>
                })}
            </div>
        </div>
    }
}