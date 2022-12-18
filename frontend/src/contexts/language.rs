use yew::prelude::*;
use yew::suspense::use_future;

use crate::components::loading::Loading;
use crate::hooks::LanguageContext;
use crate::hooks::LanguageKind;
use crate::services::users::get_user_preferences;
use crate::types::UserPreferences;

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct LanguageProviderProps {
    pub children: Children,
}

#[function_component(LanguageProvider)]
pub(crate) fn language_provider(props: &LanguageProviderProps) -> Html {
    let loading = use_state(|| true);

    let user_preferences =
        use_future(|| async { get_user_preferences().await.unwrap_or_default() });

    let user_preferences = match user_preferences {
        Ok(users) => users.clone(),
        Err(_) => UserPreferences::default(),
    };

    let language_ctx = use_state(|| LanguageKind::English);

    {
        let loading = loading.clone();
        let language_ctx = language_ctx.clone();
        use_effect_with_deps(
            move |user_preferences| {
                let language = match &user_preferences.locale {
                    Some(locale) => match LanguageKind::from_str(&locale) {
                        Some(language) => language,
                        None => LanguageKind::English,
                    },
                    None => LanguageKind::English,
                };
                log::info!("Language: {:?}", language);
                language_ctx.set(language);
                loading.set(false);
                || {}
            },
            user_preferences,
        );
    }

    let language_ctx = LanguageContext::new(language_ctx);

    if *loading {
        html! {
            <div>
                <Loading />
            </div>
        }
    } else {
        html! {
            <ContextProvider<LanguageContext> context={language_ctx}>
                {props.children.clone()}
            </ContextProvider<LanguageContext>>
        }
    }
}
