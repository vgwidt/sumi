use crate::hooks::use_language_context;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub time: chrono::NaiveDateTime,
}

#[function_component(TimeFormat)]
pub fn time_format(props: &Props) -> HtmlResult {
    let language = use_language_context()?;

    let now = chrono::Utc::now().naive_utc();

    let diff = now - props.time;

    let minutes = diff.num_minutes();
    let hours = diff.num_hours();
    let days = diff.num_days();

    let timestring = {
        if minutes < 1 {
            language.get("Just now")
        } else if minutes == 1 {
            format!("{}{}", minutes, language.get(" minute ago"))
        } else if minutes < 60 {
            format!("{}{}", minutes, " minutes ago")
        } else if hours == 1 {
            format!("{}{}", hours, " hour ago")
        } else if hours < 24 {
            format!("{}{}", hours, " hours ago")
        } else if days == 1 {
            format!("{}{}", days, " day ago")
        } else if days < 7 {
            format!("{}{}", days, " days ago")
        } else if days < 30 {
            format!("{}{}", days / 7, " weeks ago")
        } else if days < 365 {
            format!("{}{}", days / 30, " months ago")
        } else {
            format!("{}{}", days / 365, " years ago")
        }
    };

    Ok(html! {
        <span class="timeformat">
            {&timestring}
        </span>
    })
}
