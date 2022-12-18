use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ThemeKind {
    Dark,
    Light,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
    pub fn current(&self) -> &Theme {
        static LIGHT_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "#212124".to_string(),
            background_color: "#dededb".to_string(),
            nav_background_color: "#cdcdce".to_string(),
            link_color: "#282830".to_string(),
            logo_inversion: "0%".to_string(),
            table_header_color: "#d2d2d4".to_string(),
            edit_background_color: "#ebebee".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "#dededb".to_string(),
            background_color: "#212124".to_string(),
            nav_background_color: "#171718".to_string(),
            link_color: "rgb(175, 184, 221)".to_string(),
            logo_inversion: "100%".to_string(),
            table_header_color: "#1b1b1c".to_string(),
            edit_background_color: "#191919".to_string(),
        });

        match self {
            ThemeKind::Dark => &DARK_THEME,
            ThemeKind::Light => &LIGHT_THEME,
        }
    }

    // for when we have more than two themes
    // pub fn iter() -> impl Iterator<Item = ThemeKind> {
    //     vec![ThemeKind::Dark, ThemeKind::Light].into_iter()
    // }

    // pub fn label(&self) -> &str {
    //     match self {
    //         ThemeKind::Dark => "Dark",
    //         ThemeKind::Light => "Light",
    //     }
    // }

    // pub fn value(&self) -> &str {
    //     match self {
    //         ThemeKind::Dark => "dark",
    //         ThemeKind::Light => "light",
    //     }
    // }
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
    pub font_color: String,
    pub background_color: String,
    pub nav_background_color: String,
    pub link_color: String,
    pub logo_inversion: String,
    pub table_header_color: String,
    pub edit_background_color: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: ThemeKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> ThemeKind {
        (*self.inner).clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &*self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[styled_component(ThemeProvider)]
pub(crate) fn theme_provider(props: &ThemeProviderProps) -> Html {
    let theme_kind = use_state(|| get_theme());

    let theme_ctx = ThemeContext::new(theme_kind);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}

//get_theme, we should get this from user settings but for now default to user's system theme
fn get_theme() -> ThemeKind {
    let window = web_sys::window().unwrap();
    let media_query = window
        .match_media("(prefers-color-scheme: dark)")
        .unwrap()
        .unwrap();
    if media_query.matches() {
        ThemeKind::Dark
    } else {
        ThemeKind::Light
    }
}
