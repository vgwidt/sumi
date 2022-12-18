use yew::prelude::*;

use crate::components::loading::Loading;
use crate::services::auth::*;
use crate::types::{Error, MyUser};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(MyUser::default);
    let loading = use_state(|| true);

    {
        let user_ctx = user_ctx.clone();
        let loading = loading.clone();
        use_effect_with_deps(
            move |_| {
                let user_ctx = user_ctx.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = current().await;
                    if let Ok(user) = result {
                        user_ctx.set(user);
                    } else if let Err(err) = result {
                        match err {
                            Error::Unauthorized | Error::Forbidden => {
                                user_ctx.set(MyUser::default());
                            }
                            _ => (),
                        }
                    }
                    loading.set(false);
                });
                || {}
            },
            (),
        );
    }

    if *loading {
        html! {
            <div>
                <Loading />
            </div>
        }
    } else {
        html! {
            <ContextProvider<UseStateHandle<MyUser>> context={user_ctx}>
            { for props.children.iter() }
             </ContextProvider<UseStateHandle<MyUser>>>
        }
    }
}
