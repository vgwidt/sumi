use yew::prelude::*;

use crate::hooks::{use_language_context, use_user_context};
use crate::services::auth::*;

//Logout button
#[function_component(Logout)]
pub fn logout_button() -> HtmlResult {
    let user_ctx = use_user_context();
    let language = use_language_context()?;

    let onclick = {
        let logout = logout.clone();
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result = logout().await;
                if let Err(err) = result {
                    log::error!("Logout error: {:?}", err);
                }
            });
            user_ctx.ctx_logout();
        })
    };

    //button
    Ok(html! {
        <button class="btn" {onclick}>
            { language.get("Logout") }
        </button>
    })
}
