use crate::contexts::theme;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub message: String,
    pub callback: Callback<bool>,
}

#[styled_component(Confirmation)]
pub fn confirmation(props: &Props) -> Html {
    let theme = theme::use_theme();

    let style = style!(
        r#"
        .confirmation {
            position: fixed;
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            background-color: ${bg};
            border-radius: 5px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.2);
            padding: 20px;
            width: 300px;
            text-align: center;
            font-size: 1.2rem;
            font-weight: 600;
            color: ${text};
        }
        "#,
        bg = theme.secondary_background.clone(),
        text = theme.font_color.clone(),
    )
    .expect("Failed to parse style");

    //onclick will return true or false
    let onclick_true = {
        let callback = props.callback.clone();
        Callback::from(move |_| {
            callback.emit(true);
        })
    };

    let onclick_false = {
        let callback = props.callback.clone();
        Callback::from(move |_| {
            callback.emit(false);
        })
    };

    html! {
        <div class={style}>
            <div class="confirmation">
                <div class="confirmation-text">{&props.message}</div>
                <div class="confirmation-buttons">
                <form>
                    <button class="btn" type="submit" value="true" onclick={onclick_true}>{ "Yes" }</button>
                    <button class="btn" type="submit" value ="false"  onclick={onclick_false.clone()}>{ "No" }</button>
                </form>
                </div>
              </div>
        </div>
    }
}
