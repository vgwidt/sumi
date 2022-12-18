use stylist::{style, yew::styled_component};
use theme::{use_theme, ThemeKind};
use yew::{html, Callback, Html};

use crate::contexts::theme;

#[styled_component(ThemeToggle)]
pub(crate) fn theme_toggle() -> Html {
    let theme = use_theme();

    let on_click = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(match theme.kind() {
                ThemeKind::Light => ThemeKind::Dark,
                ThemeKind::Dark => ThemeKind::Light,
            })
        })
    };

    let style = style!(
        r#"
        &:hover {
            cursor: pointer;
            -webkit-touch-callout: none;
            -webkit-user-select: none;
            -khtml-user-select: none;
            -moz-user-select: none;
            -ms-user-select: none;
            user-select: none;
        }
        "#
    )
    .expect("Failed to parse style");

    html! {
        <a class={style} onclick={on_click}>
            {match theme.kind() {
                ThemeKind::Light => "🌙",
                ThemeKind::Dark => "☀️",
            }}
        </a>
    }
}
