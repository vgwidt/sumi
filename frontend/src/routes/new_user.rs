use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::hooks::use_language_context;
use crate::routes::AppRoute;
use crate::services::users::create;
use crate::types::RegisterInfo;

//Create new user page
//Considering combining into settings, but may be better to refactor components
#[function_component(NewUser)]
pub fn new_user() -> Html {
    let language = use_language_context();
    let navigator = use_navigator().unwrap();
    let register_info = use_state(RegisterInfo::default);
    let submitted = use_state(|| false);
    let error = use_state(|| String::new());

    {
        let register_info = register_info.clone();
        let navigator = navigator.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |submitted| {
                if *submitted {
                    wasm_bindgen_futures::spawn_local(async move {
                        let request = RegisterInfo {
                            username: register_info.username.clone(),
                            display_name: register_info.display_name.clone(),
                            email: register_info.email.clone(),
                            password: register_info.password.clone(),
                            access: "1".to_string(),
                        };
                        let result = create(request).await;
                        if let Err(err) = result {
                            log::error!("Create user error: {:?}", err);
                            error.set(err.to_string());
                        } else if result.is_ok() {
                            navigator.push(&AppRoute::Users);
                        } else {
                            log::error!("Create user failed without an error");
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
        let error = error.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            error.set(String::new());
            submitted.set(true);
        })
    };
    let oninput_username = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.username = input.value();
            register_info.set(info);
        })
    };
    let oninput_display_name = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.display_name = input.value();
            register_info.set(info);
        })
    };
    let oninput_email = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.email = input.value();
            register_info.set(info);
        })
    };
    let oninput_password = {
        let register_info = register_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*register_info).clone();
            info.password = input.value();
            register_info.set(info);
        })
    };

    html! {
        <div>
            <h1>{ "Add User" }</h1>
            <div class="error">
                {error.to_string()}
            </div>
            <form {onsubmit}>
                <fieldset>
                    <div>
                        <label>{ format!("{}:", language.get("Username")) }</label>
                        <input
                            type="text"
                            placeholder={language.get("Username")}
                            value={register_info.username.clone()}
                            oninput={oninput_username}
                            />
                        </div>
                    <div>
                        <label>{ format!("{}:", language.get("Display Name")) }</label>
                        <input
                            type="text"
                            placeholder={language.get("Display Name")}
                            value={register_info.display_name.clone()}
                            oninput={oninput_display_name}
                            />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("E-mail")) }</label>
                        <input
                            type="email"
                            placeholder={language.get("Email")}
                            value={register_info.email.clone()}
                            oninput={oninput_email}
                            />
                    </div>
                    <div>
                        <label>{ format!("{}:", language.get("Password")) }</label>
                        <input
                            type="password"
                            placeholder={language.get("Password")}
                            value={register_info.password.clone()}
                            oninput={oninput_password}
                            />
                    </div>
                    <button
                        class="btn"
                        type="submit"
                        disabled=false>
                        { "Create" }
                    </button>
                </fieldset>
            </form>
        </div>
    }
}
