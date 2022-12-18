pub mod document;
mod tree;

use stylist::style;
use stylist::yew::styled_component;
use uuid::Uuid;
use yew::{html, use_state, Callback, Html, Properties};

use crate::routes::wiki::document::WikiDocument;
use crate::routes::wiki::tree::WikiTree;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub document_id: Option<Uuid>,
}

#[styled_component(Wiki)]
pub fn wiki(props: &Props) -> Html {
    let uptodate = use_state(|| true);

    let style = style! {
        r#"
        .wiki {
            display: flex;
            flex-direction: row;
            height: 100%;
            width: 100%;
        }
        .wiki-tree {
            flex-grow: 1;
            max-width: 300px;
            overflow: auto;
            height: 100%;
            border-right: 1px solid #777;
            font-size: 1.2em;
            width: 30%;
        }
        .wiki-tree ul {
            list-style: none;
            padding: 2px 0 0px 20px;
        }
        .wiki-tree li {
            list-style: none;
            padding: 2px 0 2px 0px;
        }     
        .wiki-tree summary {
            margin-left: -16px;
        }   
        "#
    }
    .expect("Failed to parse style");

    let callback_changed = {
        let uptodate = uptodate.clone();
        Callback::from(move |_| uptodate.set(false))
    };

    let callback_updated = {
        let uptodate = uptodate.clone();
        Callback::from(move |_| uptodate.set(true))
    };

    html! {
        <div class={style}>
            <div class="wiki">
                <WikiTree uptodate={*uptodate} updated={callback_updated} />
                <WikiDocument document_id={props.document_id.clone()} needs_update={callback_changed} />
            </div>
        </div>
    }
}
