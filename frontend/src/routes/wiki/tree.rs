use stylist::yew::styled_component;

use uuid::Uuid;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew::virtual_dom::VNode;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::services::documents::get_doc_tree;
use crate::types::DocumentMetadata;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub document_id: Option<Uuid>,
    pub uptodate: bool,
    pub updated: Callback<bool>,
}

#[styled_component(WikiTree)]
pub fn wiki_tree(props: &Props) -> Html {
    let documents = match use_future_with(props.uptodate, |_| {
        props.updated.emit(true);
        get_doc_tree()
    }) {
        Ok(documents) => documents,
        Err(_) => {
            return html! {
                <div class="wiki-tree">
                </div>
            }
        }
    };

    html! {
        <div class="wiki-tree">
            <ul>
                {render_documents(&documents.as_ref().unwrap_or(&Vec::new()), None, &props)}
            </ul>
        </div>
    }
}

fn render_documents(
    documents: &Vec<DocumentMetadata>,
    parent_id: Option<Uuid>,
    props: &Props,
) -> Vec<VNode> {
    let mut children = Vec::new();

    //Loops through the documents, starting with the root documents
    //If the document's parent_id is the same as the parent_id passed to the function, add it to the list
    //this means that the document is a child of the parent_id
    for document in documents {
        if document.parent_id == parent_id {
            let mut child = html! {
                <li>
                    <Link<AppRoute> to={AppRoute::WikiDoc { document_id: document.document_id.clone() }} classes={
                        if props.document_id == Some(document.document_id) {
                            "selected nav-link"
                        } else {
                            "nav-link"
                        }
                        }>
                        {&document.title}
                    </Link<AppRoute>>
                </li>
            };

            //search subdocuments of the current document
            let sub_children = render_documents(documents, Some(document.document_id), props);

            if !sub_children.is_empty() {
                child = html! {
                    <li>
                        <details>
                            <summary>
                                <Link<AppRoute> to={AppRoute::WikiDoc { document_id: document.document_id.clone() }} classes={
                                    if props.document_id == Some(document.document_id) {
                                        "selected nav-link"
                                    } else {
                                        "nav-link"
                                    }
                                    }>
                                    {&document.title}
                                </Link<AppRoute>>
                            </summary>

                        <ul>
                            {sub_children}
                        </ul>
                        </details>
                    </li>
                };
            }

            children.push(child);
        }
    }

    children
}
