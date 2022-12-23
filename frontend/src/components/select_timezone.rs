use crate::hooks::{use_language_context, use_user_context};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{services::users::update_user_preferences, types::UserPreferences};

#[function_component(SelectTimezone)]
pub fn select_timezone() -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();

    let onclick = {
        let user_ctx = user_ctx.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let prefs = UserPreferences {
                    timezone: Some(value),
                    ..Default::default()
                };
                update_user_preferences(prefs).await.unwrap();
            });
            user_ctx.update_timezone(input.value());
        })
    };


    html!(
        <form>
            <label>
            {language.get("Timezone")}{": "}</label>
            <select 
            onchange={onclick}>
                {
                    for chrono_tz::TZ_VARIANTS.iter().map(|x| {
                        html!(
                            <option value={x.to_string()}
                            selected={user_ctx.timezone.clone().as_deref().unwrap_or(&"UTC".to_string()) == x.to_string()}
                            >{x.to_string()}</option>
                        )
                    })
                }
            </select>
        </form>

    )
}
