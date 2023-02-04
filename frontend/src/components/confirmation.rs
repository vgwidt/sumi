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

    let page_cover = style!(
        r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.5);
        z-index: 10;
        "#,
    ).expect("Failed to parse style");

    let style = style!(
        r#"
        position: absolute;
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
            z-index: 11;
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

    let onclick = {
        let callback = props.callback.clone();
        Callback::from(move |_| {
            callback.emit(false);
        })
    };


    html! {
        //cover whole page with invisible div to close dropdown when clicking outside of it
        <>
        <div class={page_cover} onclick={onclick}></div>
        <div class={style}>
            <div class="confirmation">
                <div class="confirmation-text">{&props.message}</div>
                <div class="confirmation-buttons">
                <form>
                    <button class="btn" type="submit" value="true" onclick={onclick_true}>{ "Yes" }</button>
                    <button class="btn" type="submit" value ="false"  onclick={onclick_false}>{ "No" }</button>
                </form>
                </div>
              </div>
        </div>
        </>
    }
}
