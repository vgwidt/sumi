use crate::hooks::{get_language_list, use_language_context, LanguageKind};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{services::users::update_user_preferences, types::UserPreferences};

#[function_component(SelectLanguage)]
pub fn select_language() -> Html {
    let locale = use_language_context();

    //get list of locales available from LanguageKind in locale.rs
    let options = get_language_list();

    let onclick = {
        let locale = locale.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            locale.set(match value.as_str() {
                "en_US" => LanguageKind::English,
                "ja_JP" => LanguageKind::Japanese,
                _ => LanguageKind::English,
            });
            wasm_bindgen_futures::spawn_local(async move {
                let prefs = UserPreferences {
                    locale: Some(input.value().to_string()),
                    ..Default::default()
                };
                update_user_preferences(prefs).await.unwrap();
            });
        })
    };

    html!(
        <form>
            <label>
            {locale.get("Language")}{": "}</label>
            <select onchange={onclick}>
                { for options.iter().map(|option| html!(<option value={option.value()}
                selected={if locale.kind().value() == option.value() {true} else {false}}>
                {option.label()}</option>)) }
            </select>
        </form>

    )
}
