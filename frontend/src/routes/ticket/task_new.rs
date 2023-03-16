use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use shared::models::tasks::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub ticket_id: i32,
    pub task: TaskNewPayload,
    pub callback: Callback<String>,
    pub callback_updated: Callback<TaskRepresentation>,
}

#[styled_component(NewTask)]
pub fn new_task(props: &Props) -> Html {
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());


    html! {
        <div class="task">
            <input type="checkbox" checked={props.task.is_done}/>
            <input type="text" value={props.task.label.clone()} placeholder="New task" />
        </div>
    }

}