use shared::models::MAX_TITLE_LENGTH;
use stylist::style;
use stylist::yew::styled_component;

use uuid::Uuid;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::delete::{DeleteItem, ItemTypes};
use crate::hooks::{use_language_context, use_user_context};
use crate::routes::wiki::revision_list::Revisions;
use crate::routes::AppRoute;
use crate::services::documents::{create_document, get_document, update_document};
use crate::types::{DocumentCreateInfo, DocumentUpdateInfo};
use crate::utils::markdown_to_html;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub document_id: Option<Uuid>,
    pub needs_update: Callback<bool>,
}

#[styled_component(WikiDocument)]
pub fn wiki_document(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();
    let navigator = use_navigator().unwrap();
    let update_info = use_state(DocumentUpdateInfo::default);
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());
    //is_new is set to true when create button is clicked (for using create vs update service)
    let is_new = use_state(|| false);
    let edit_mode = use_state(|| false);
    let view_revisions = use_state(|| false);

    //Reruns on edit as a workaround for when editing is cancelled, since the html displays update_info
    //It might be better to have a different state that holds the original values
    {
        let update_info = update_info.clone();
        let document_id = props.document_id.clone();
        let is_new = is_new.clone();
        use_effect_with((props.document_id.clone(), edit_mode.clone()), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(id) = document_id {
                    if *is_new {
                        return;
                    } else {
                        let result = get_document(&id).await;
                        if let Ok(document) = result {
                            update_info.set(DocumentUpdateInfo {
                                title: Some(document.title.clone()),
                                content: Some(document.content.clone()),
                                parent_id: Some(document.parent_id.clone()),
                                archived: Some(document.archived),
                                version: Some(document.updated_at),
                            });
                        }
                    }
                } else {
                    update_info.set(DocumentUpdateInfo::default());
                }
            });
            || ()
        });
    }

    //set edit_mode to false when props.document_id changes (i.e. clicks on a different document)
    {
        let edit_mode = edit_mode.clone();
        let is_new = is_new.clone();
        use_effect_with(props.document_id.clone(), move |document_id| {
            if document_id.is_some() {
                is_new.set(false);
                edit_mode.set(false);
            }
            || ()
        });
    }

    //clear update_info when props.document_id changes
    {
        let update_info = update_info.clone();
        use_effect_with(props.document_id.clone(), move |document_id| {
            if document_id.is_some() {
                update_info.set(DocumentUpdateInfo::default());
            }
            || ()
        });
    }

    //Send update_info to server when submitted is true
    {
        let document_id = props.document_id.clone();
        let update_info = update_info.clone();
        let user_ctx = user_ctx.clone();
        let is_new = is_new.clone();
        let edit_mode = edit_mode.clone();
        let error = error.clone();
        let submitted = submitted.clone();
        let navigator = navigator.clone();
        let props = props.clone();
        use_effect_with(submitted.clone(), move |submitted| {
            if **submitted {
                wasm_bindgen_futures::spawn_local(async move {
                    //If it is an existing document, we only adjust the updated_by field
                    let result = {
                        if *is_new == false {
                            let request = DocumentUpdateInfo {
                                title: update_info.title.clone(),
                                content: update_info.content.clone(),
                                parent_id: update_info.parent_id.clone(),
                                archived: update_info.archived,
                                version: update_info.version,
                            };
                            update_document(&document_id.unwrap_or_default(), request).await
                        } else {
                            //New document also gets created_by adjusted
                            let request = DocumentCreateInfo {
                                title: update_info.title.clone().unwrap_or_default(),
                                content: update_info.content.clone().unwrap_or_default(),
                                parent_id: update_info.parent_id.clone().unwrap_or_default(),
                                created_by: user_ctx.user_id.clone(),
                                updated_by: user_ctx.user_id.clone(),
                            };
                            create_document(request).await
                        }
                    };
                    if let Ok(document) = result {
                        if document.success {
                            edit_mode.set(false);
                            is_new.set(false);
                            props.needs_update.emit(true);
                            navigator.push(&AppRoute::WikiDoc {
                                document_id: document.data.unwrap().document_id,
                            });
                        } else {
                            error.set(document.message.unwrap_or("Unknown error".to_string()));
                        }
                    } else {
                        error.set(result.err().unwrap().to_string());
                    }
                });
                submitted.set(false);
            }
            || ()
        });
    }

    let oninput_title = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.title = Some(input.value());
            update_info.set(info);
        })
    };

    let oninput_content = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.content = Some(input.value());
            update_info.set(info);
        })
    };

    let on_submit = {
        let submitted = submitted.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            submitted.set(true);
        })
    };

    let on_click_cancel = {
        let edit_mode = edit_mode.clone();
        let is_new = is_new.clone();
        Callback::from(move |_| {
            is_new.set(false);
            edit_mode.set(false);
        })
    };

    let on_click_edit = {
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            edit_mode.set(true);
        })
    };

    let onclick_create = {
        let document_id = props.document_id.clone();
        let update_info = update_info.clone();
        let edit_mode = edit_mode.clone();
        Callback::from(move |_| {
            is_new.set(true);
            let info = DocumentUpdateInfo {
                parent_id: Some(document_id),
                ..Default::default()
            };
            update_info.set(info);
            edit_mode.set(true);
        })
    };

    let callback_deleted = {
        let props = props.clone();
        Callback::from(move |_| {
            props.needs_update.emit(true);
            navigator.push(&AppRoute::WikiHome);
        })
    };

    // let onclick_revisions = { //opens revisions component by setting view_revisions to true
    //     let view_revisions = view_revisions.clone();
    //     Callback::from(move |_| {
    //         if *view_revisions {
    //             view_revisions.set(false);
    //         } else {
    //             view_revisions.set(true);
    //         }
    //     })
    // };

    let style = style! {
        r#"
        min-width: 400px;
        width: 100%;
        .wiki-document {
            word-wrap: break-word;
            min-width: 300px;
            height: 100%;
            overflow: auto;
            margin-left: 32px;
            margin-right: 32px;
        }
        .title-input {
            width: 100%;
            padding: 12px;
            margin-bottom: 12px;
        }
        textarea {
            width: 100%;
            padding: 12px;
        }
        "#
    }
    .expect("Failed to parse style");

    html! {
        <div class={style}>
            <div class="error">
                {error.to_string()}
            </div>
            {
                if *edit_mode {
                    html! {
                        <div class="wiki-document">
                            <form class="wiki-form" onsubmit={on_submit}>
                                <fieldset style="border: none;">
                                    <input class="title-input" type="text" placeholder="Title" value={update_info.title.clone()} oninput={oninput_title} maxlength={MAX_TITLE_LENGTH.to_string()}/>
                                    <div class="wiki-content">
                                        <textarea placeholder="Description (Markdown)" rows=12 value={update_info.content.clone()} oninput={oninput_content} />
                                        </div>
                            <div class="wiki-buttons">
                                <button class="btn" type="submit">
                                    {language.get("Save")}
                                </button>
                                <button class="btn" onclick={on_click_cancel}>
                                    {language.get("Cancel")}
                                </button>
                            </div>
                        </fieldset>
                        </form>
                        </div>
                    }
                } else {
                    if let Some(document_id) = props.document_id {
                    html! {
                        <div class="wiki-document">
                            <div class="wiki-buttons">
                                <button class="btn" onclick={onclick_create}
                                    title={language.get("Create a new nested document")}>
                                    {language.get("Create")}
                                </button>
                                <button class="btn" onclick={on_click_edit}>
                                    {language.get("Edit")}
                                </button>
                                <DeleteItem item_id={document_id.to_string()} item_type={ItemTypes::Document}
                                    callback={callback_deleted} />
                                // <button class="btn" onclick={onclick_revisions}>
                                //     {language.get("Revisions")}
                                // </button>
                            </div>
                            <h1 class="wiki_title">
                                {update_info.title.clone()}
                            </h1>
                            <div class="wiki_content">
                                {markdown_to_html(&update_info.content.clone().unwrap_or_default())}
                            </div>
                            { if *view_revisions {
                                html! {
                                    <Revisions id={document_id} />
                                }
                            } else {
                                html! {}
                            }}

                        </div>
                    }
                }
                else {
                    html! {
                    <div class="wiki-document">
                        <div class="wiki-buttons">
                            <button class="btn" onclick={onclick_create}>
                                {language.get("Create")}
                            </button>
                        </div>
                    </div>
                    }

                }
            }
    }
    </div>
    }
}
