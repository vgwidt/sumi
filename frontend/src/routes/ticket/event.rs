use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::time_format::TimeFormat;
use crate::types::events::TicketEvent;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub event: TicketEvent,
}

#[styled_component(EventCard)]
pub fn event(props: &Props) -> Html {
    let event = &props.event;

    html!{
        <div class="event-card">
            <TimeFormat time={event.created_at.clone()}/>
            {format!(": {}", event.to_string())}
        </div>
    }
}
