use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{collections::HashMap, ops::Deref};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Strings {
    pub strings: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub(crate) enum LanguageKind {
    #[default]
    English,
    Japanese,
}

impl LanguageKind {
    pub fn current(&self) -> &Language {
        match &self {
            LanguageKind::English => {
                static ENGLISH_LANGUAGE: Lazy<Language> = Lazy::new(|| Language {
                    strings: serde_json::from_str(include_str!("../i18n/en_US.json"))
                        .unwrap_or_default(),
                });
                &ENGLISH_LANGUAGE
            }
            LanguageKind::Japanese => {
                static JAPANESE_LANGUAGE: Lazy<Language> = Lazy::new(|| Language {
                    strings: serde_json::from_str(include_str!("../i18n/ja_JP.json"))
                        .unwrap_or_default(),
                });
                &JAPANESE_LANGUAGE
            }
        }
    }

    pub fn iter() -> impl Iterator<Item = LanguageKind> {
        vec![LanguageKind::English, LanguageKind::Japanese].into_iter()
    }

    pub fn label(self) -> &'static str {
        match self {
            LanguageKind::English => "English",
            LanguageKind::Japanese => "日本語",
        }
    }

    pub fn value(self) -> &'static str {
        match self {
            LanguageKind::English => "en_US",
            LanguageKind::Japanese => "ja_JP",
        }
    }

    pub fn from_str(s: &str) -> Option<LanguageKind> {
        match s {
            "en_US" => Some(LanguageKind::English),
            "ja_JP" => Some(LanguageKind::Japanese),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Language {
    pub strings: HashMap<String, String>,
}

impl Language {
    /// Searches using the key and returns the value if found
    /// If not found, falls back to the key
    pub fn get(&self, key: &str) -> String {
        self.strings
            .iter()
            .find(|(k, _)| k == &key)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| key.to_string())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LanguageContext {
    inner: UseStateHandle<LanguageKind>,
}

impl LanguageContext {
    pub fn new(inner: UseStateHandle<LanguageKind>) -> Self {
        Self { inner }
    }

    pub fn set(&self, kind: LanguageKind) {
        self.inner.set(kind)
    }

    pub fn kind(&self) -> LanguageKind {
        (*self.inner).clone()
    }
}

impl Deref for LanguageContext {
    type Target = Language;

    fn deref(&self) -> &Self::Target {
        &*self.inner.current()
    }
}

impl PartialEq for LanguageContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[hook]
pub(crate) fn use_language_context() -> yew::suspense::SuspensionResult<LanguageContext> {
    match use_context::<LanguageContext>() {
        Some(ctx) => Ok(ctx),
        None => {
            let (s, handle) = yew::suspense::Suspension::new();
            Err(s)
        }
    }
}

pub(crate) fn get_language_list() -> Vec<LanguageKind> {
    LanguageKind::iter().collect()
}
