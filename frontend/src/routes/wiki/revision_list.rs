//gets the revisions of document or ticket

use stylist::{style, yew::styled_component};
use uuid::Uuid;
use yew::{prelude::*, suspense::use_future_with};

use crate::{
    services::documents::document_revisions, types::DocumentRevision, utils::markdown_to_html,
};

//props accepts ID of document or ticket (since different types, use T)
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Uuid,
}

#[styled_component(Revisions)]
pub fn revisions(props: &Props) -> Html {
    let revisions: UseStateHandle<Vec<DocumentRevision>> = use_state(|| vec![]);
    let error = use_state(|| String::new());

    {
        let revisions = revisions.clone();
        let props = props.clone();
        match use_future_with(props.id.clone(), move |_| async move {
            let result = document_revisions(props.id).await;
            match result {
                Ok(r) => {
                    revisions.set(r);
                }
                Err(e) => {
                    error.set(e.to_string());
                }
            }
        }) {
            Ok(_) => (),
            Err(_) => (),
        }
    }

    //show on the right side of the documents page when it is opened
    let style = style!(
        r#"
            position: fixed;
            top: 0;
            right: 0;
            width: 20%;
            height: 100%;
            padding: 1rem;
            box-shadow: -5px 0 10px rgba(0, 0, 0, 0.2);
            overflow-y: auto;
        "#
    )
    .expect("Failed to parse style");

    {
        let revisions = &*revisions.clone();
        html! {
            <div class={style}>
                <h2>{"Revision History"}</h2>
                <ul>
                    { for revisions.iter().map(|revision| html! {
                        <li>
                            <p>{ markdown_to_html(&revision.content) }</p>
                            <p>{ &format!("Updated at {}", &revision.updated_at) }</p>
                        </li>
                    })}
                </ul>
            </div>
        }
    }
}
