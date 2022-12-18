use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(Loading)]
pub fn loading() -> Html {
    let style = style!(
        r#"
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        "#
    )
    .expect("Failed to parse style");

    html! {
        <div class={style}>
            <img src="/img/Enso.svg" alt="Logo" />
        </div>
    }
}
