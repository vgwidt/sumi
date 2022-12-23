use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::components::logout::Logout;
use crate::components::select_locale::SelectLanguage;
use crate::components::select_timezone::SelectTimezone;
use crate::hooks::{use_language_context, use_user_context};
use crate::services::users::*;
use crate::types::UserUpdateInfo;

use super::AppRoute;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub user_id: Option<uuid::Uuid>,
}

/// Update user settings
#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());

    //if props.id is None, then we are updating the current user
    let user_id = props.user_id.clone().unwrap_or(user_ctx.user_id.clone());
    let navigator = use_navigator().unwrap();
    let update_info = use_state(UserUpdateInfo::default);
    let password = use_state(String::default);
    let password_confirm = use_state(String::default);

    //Get user info from server, set values except password
    {
        let user_id = user_id.clone();
        let update_info = update_info.clone();
        use_effect_with_deps(
            move |_| {
                let update_info = update_info.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = get_userinfo(user_id).await.unwrap();
                    update_info.set(UserUpdateInfo {
                        email: result.email.clone(),
                        username: result.username.clone(),
                        display_name: result.display_name.clone(),
                        access: result.access.clone(),
                        password: None,
                    })
                });
                || ()
            },
            user_id.clone(),
        )
    }

    {
        let update_info = update_info.clone();
        let error = error.clone();
        let password = password.clone();
        let user_id = user_id.clone();
        use_effect_with_deps(
            move |submitted| {
                if *submitted {
                    wasm_bindgen_futures::spawn_local(async move {
                        let mut request = UserUpdateInfo {
                            username: update_info.username.clone(),
                            display_name: update_info.display_name.clone(),
                            email: update_info.email.clone(),
                            access: update_info.access.clone(),
                            password: update_info.password.clone(),
                        };
                        if !(*password).is_empty() {
                            request.password = Some((*password).clone());
                        }
                        let result = save(user_id, request).await;
                        if let Err(err) = result {
                            log::error!("Update user error: {:?}", err);
                            error.set(err.to_string());
                        } else if let Ok(user_info) = result {
                            if user_info.user_id == user_ctx.user_id {
                                user_ctx.update_info(user_info.clone());
                            }
                            navigator.push(&AppRoute::Users);
                        } else {
                            log::error!("Update user failed without an error");
                        }
                    });
                }
                || {}
            },
            *submitted.clone(),
        );
    }

    {
        let submitted = submitted.clone();
        use_effect_with_deps(
            move |_| {
                submitted.set(false);
                || {}
            },
            error.clone(),
        );
    }

    let onsubmit = {
        //verify passwords match
        let password = password.clone();
        let password_confirm = password_confirm.clone();
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            if (*password) != (*password_confirm) {
                e.prevent_default();
                return;
            }
            e.prevent_default();
            error.set(String::new());
            submitted.set(true);
        })
    };
    let oninput_username = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.username = input.value();
            update_info.set(info);
        })
    };
    let oninput_display_name = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.display_name = input.value();
            update_info.set(info);
        })
    };
    let oninput_email = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.email = input.value();
            update_info.set(info);
        })
    };
    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };
    let oninput_password_confirm = {
        let password_confirm = password_confirm.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password_confirm.set(input.value());
        })
    };

    html! {
        <div class="settings-page">
            <h1>{ language.get("Settings") }</h1>
            <div class="error">
                {error.to_string()}
            </div>
            <form {onsubmit}>
                <fieldset>
                    <div>
                        <label>{ format!("{}:", language.get("Username")) }</label>
                        <input type="text" placeholder="Username" value={update_info.username.clone()}
                            oninput={oninput_username} required=true />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("Display Name")) }</label>
                        <input placeholder="Display Name" value={update_info.display_name.clone()}
                            oninput={oninput_display_name} />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("E-mail")) }</label>
                        <input type="email" placeholder="E-mail" value={update_info.email.clone()}
                            oninput={oninput_email} />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("Password")) }</label>
                        <input type="password" placeholder="New Password" value={(*password).clone()}
                            oninput={oninput_password} />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("Confirm Password")) }</label>
                        <input type="password" placeholder="Confirm Password" value={(*password_confirm).clone()}
                            oninput={oninput_password_confirm} />
                    </div>
                    <button class="btn" type="submit">
                        //disabled={user_info.loading || user_update.loading}>
                        { language.get("Save") }
                    </button>
                </fieldset>
            </form>
            <hr />
            // only show if no user_id (indicating current user settings)
            { if props.user_id.is_none() {
                html! {
                    <div>
                        <SelectLanguage />
                        <SelectTimezone />
                        <hr />
                        <Logout />
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
