use shared::models::tasks::TaskRepresentation;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use shared::models::tasks::*;

use crate::services::tasks::create_task;

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
    let update_info = use_state(|| props.task.clone());

    // let onclick_save = {
    //     let props = props.clone();
    //     let submitted = submitted.clone();
    //     let error = error.clone();
    //     let update_info = update_info.clone();
    //     Callback::from(move |event: MouseEvent| {
    //         let props = props.clone();
    //         let submitted = submitted.clone();
    //         let error = error.clone();
    //         let new_task = TaskNewPayload {
    //             //todo
                
    //         };
    //         wasm_bindgen_futures::spawn_local(async move {
    //             submitted.set(true);
    //             let result = create_task(props.ticket_id, new_task.clone()).await;
    //             match result {
    //                 Ok(task) => {
    //                     submitted.set(false);
    //                     //props.callback_updated.emit(task);
    //                 }
    //                 Err(e) => {
    //                     submitted.set(false);
    //                     error.set(e.to_string());
    //                 }
    //             }
    //         });
    //     })
    // };

    // let oninput_label = {
    //     let update_info = update_info.clone();
    //     Callback::from(move |event: InputData| {
    //         let value = event.value;
    //         let mut update_info = update_info.clone();
    //         update_info.label = value;
    //     })
    // };
    
    html! {
        <div class="task">
            <input type="checkbox" checked={props.task.is_done}/>
            <input type="text" oninput={oninput_label} value={props.task.label.clone()} placeholder="New task" />
            <button onclick={onclick_save}>{"Save"}</button>

        </div>
    }

}