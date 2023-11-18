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
            font_color: "#181820".to_string(),
            background: "#fdfdfe".to_string(),
            secondary_background: "#f1f1f6".to_string(),
            menu_background: "#e9eaec".to_string(),
            link_color: "#282830".to_string(),
            input_background: "#FFFFFF".to_string(),
            code_background: "#eeeff7".to_string(),
            border: "#a0a7b2".to_string(),
        });

        static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
            font_color: "#dededb".to_string(),
            background: "#1A1A1B".to_string(),
            secondary_background: "#121212".to_string(),
            menu_background: "#232427".to_string(),
            link_color: "rgb(175, 184, 221)".to_string(),
            input_background: "#111111".to_string(),
            code_background: "#101010".to_string(),
            border: "#404752".to_string(),
        });

        // Old light theme
        // static WASHI_THEME: Lazy<Theme> = Lazy::new(|| Theme {
        //     font_color: "#222224".to_string(),
        //     background: "#f6f6f2".to_string(),
        //     secondary_background: "#f0f0ec".to_string(),
        //     link_color: "#282830".to_string(),
        //     input_background: "#F9F9F7".to_string(),
        //     code_background: "#ebebe7".to_string(),
        //     border: "#404752".to_string(),
        // });

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
    pub background: String,
    pub secondary_background: String,
    pub menu_background: String,
    pub link_color: String,
    pub input_background: String,
    pub code_background: String,
    pub border: String,
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
