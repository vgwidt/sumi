use yew::prelude::*;
use yew_router::prelude::Redirect;

use crate::components::ticket_list::TicketList;
use crate::hooks::use_user_context;

use super::AppRoute;

#[function_component(Home)]
pub fn home() -> Html {
    let user_ctx = use_user_context();

    //loading is done as part of user context
    if user_ctx.is_authenticated() {
        html! {
        <div>
            <TicketList />
         </div>
        }
    } else {
        html! {
         <Redirect<AppRoute> to={AppRoute::Login} />
        }
    }
}
