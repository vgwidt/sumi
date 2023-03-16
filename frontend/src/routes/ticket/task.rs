use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use shared::models::tasks::*;

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


    html! {
    }

}