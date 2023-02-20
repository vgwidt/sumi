use stylist::style;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::hooks::{use_language_context, use_user_context};
use crate::routes::AppRoute;
use crate::services::auth::*;
use crate::types::LoginInfo;

/// Login page
#[styled_component(Login)]
pub fn login_page() -> HtmlResult {
    let user_ctx = use_user_context();
    let language_ctx = use_language_context()?;
    let login_info = use_state(LoginInfo::default);
    let login_flag = use_state(|| false);
    let login_error = use_state(|| String::new());

    {
        let login_info = login_info.clone();
        let login_error = login_error.clone();
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |login_flag| {
                if *login_flag {
                    wasm_bindgen_futures::spawn_local(async move {
                        let result = login(&*login_info.clone()).await;
                        if let Err(err) = result {
                            log::error!("Login error: {:?}", err);
                            login_error.set(err.to_string());
                        } else if let Ok(user_info) = result {
                            if user_info.success {
                                wasm_bindgen_futures::spawn_local(async move {
                                    let res = current().await;
                                    if let Ok(user) = res {
                                        user_ctx.login(user);
                                    } else if let Err(err) = res {
                                        log::error!("Login error: {:?}", err);
                                    }
                                });
                            } else {
                                login_error.set(user_info.message);
                            }
                        } else {
                            log::error!("Login failed without an error");
                        }
                    });
                }
                || {}
            },
            *login_flag.clone(),
        );
    }

    {
        let login_flag = login_flag.clone();
        use_effect_with_deps(
            move |_| {
                login_flag.set(false);
                || {}
            },
            login_error.clone(),
        );
    }

    let onsubmit = {
        let login_error = login_error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            login_error.set(String::new());
            login_flag.set(true);
        })
    };

    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    let style = style!(
        r#"
        margin-left: -200px;
        padding-top: 12px;
        padding-bottom: 12px;
        border: 0;
        .sign-in-button {
            width: 100%;
            margin: 0 auto;
            padding-top: 8px;
            padding-bottom: 8px;
            display: block;
            }
            .sign-in-button:hover {
                background-color: #5243c2;
            }
            .login-form {
                width: 256px;
                margin: 0 auto;
                border: 0;
            }
            h1 {
                text-align: center;
                margin-bottom: 0px;
              }
            .form-input {
                height: 32px;
                border: 0px;
                display: block;
                width: 100%;
                margin: 0 auto;
                margin-bottom: 4px;
              }
            .error {
              margin-top: 8px;
              color: #ed3434;
              text-align: center;
            }
        "#
    )
    .expect("Failed to parse style");

    if user_ctx.is_authenticated() {
        Ok(html! {
            <Redirect<AppRoute> to={AppRoute::Home} />
        })
    } else {
        Ok(html! {
            <div class={style}>
                <h1>{ "Sign In" }</h1>
                <form {onsubmit}>
                <fieldset class="login-form">
                        <input
                            class="form-input"
                            type="text"
                            placeholder={language_ctx.get("Username")}
                            value={login_info.username.clone()}
                            oninput={oninput_username}
                            />
                        <input
                            class="form-input"
                            type="password"
                            placeholder={language_ctx.get("Password")}
                            value={login_info.password.clone()}
                            oninput={oninput_password}
                            />
                        <button
                            class="btn sign-in-button"
                            type="submit"
                            disabled=false>
                            { language_ctx.get("Login") }
                        </button>
                        <div class="error">
                            {login_error.to_string()}
                        </div>
                    </fieldset>
                </form>
            </div>
        })
    }
}
