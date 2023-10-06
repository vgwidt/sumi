use chrono::{Local, TimeZone};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(TimeContextProvider)]
pub fn time_context_provider(props: &Props) -> Html {
    let time_ctx = use_state(|| i32::default());

    let offset = Local::now().offset().local_minus_utc();

    {
        let time_ctx = time_ctx.clone();
        use_effect_with((),move |_| {
            let time_ctx = time_ctx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                time_ctx.set(offset);
            });
            || {}
        });
    }

    html! {
        <ContextProvider<UseStateHandle<i32>> context={time_ctx}>
            { for props.children.clone() }
        </ContextProvider<UseStateHandle<i32>>>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TimeContext {
    pub inner: UseStateHandle<i32>,
}

#[hook]
pub(crate) fn use_time() -> TimeContext {
    let inner = use_context::<UseStateHandle<i32>>().expect("No context found");

    TimeContext { inner }
}

impl TimeContext {
    pub fn offset(&self) -> i32 {
        *self.inner
    }
    pub fn convert_to_local(&self, time: &chrono::NaiveDateTime) -> chrono::NaiveDateTime {
        let offset = self.offset();
        let local: chrono::FixedOffset = chrono::FixedOffset::east_opt(offset).unwrap();

        local.from_utc_datetime(&time).naive_local()
    }
    pub fn convert_to_utc(&self, time: &chrono::NaiveDateTime) -> chrono::NaiveDateTime {
        let offset = self.offset();
        let local: chrono::FixedOffset = chrono::FixedOffset::east_opt(offset).unwrap();

        local.from_local_datetime(&time).unwrap().naive_utc()
    }
}