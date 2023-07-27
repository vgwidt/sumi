use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::Router;
use yew_router::prelude::{use_navigator, use_route, Link};

use crate::components::logout::Logout;
use crate::components::select_locale::SelectLanguage;
use crate::hooks::{use_language_context, use_user_context};
use crate::routes::SettingsRoute;
use crate::services::users::*;
use crate::types::UserUpdateInfo;

use super::AppRoute;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub user_id: Option<uuid::Uuid>,
}

/// Update user settings
#[function_component(Settings)]
pub fn settings(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();

    let route = match use_route::<SettingsRoute>() {
        Some(route) => route,
        None => SettingsRoute::Profile,
    };
    
    //Settings will display the nav bar, then depending on the route, will display the appropriate page (Account settings, ticket settings, etc)
    html!{
        //Match SettingsRoute to display the appropriate page
        <div class="settings">
            <div class="settings-nav">
                <div class="settings-nav-header">
                    <h1>{ "Settings" }</h1>
                </div>
                <div class="settings-nav-body">
                    <ul>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::Profile} classes="nav-link">
                                { "Profile" }
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::Tickets} classes="nav-link">
                                { "Tickets" }
                            </Link<SettingsRoute>>
                        </li>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::Account { user_id: user_ctx.user_id.clone() }} classes="nav-link">
                                { "Account" }
                            </Link<SettingsRoute>>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="settings-body">
                // { if let AppRoute::SettingsRoot = route {
                //     html!{
                //         <div class="settings-body-header">
                //             <h1>{ "Settings Root" }</h1>
                //         </div>
                //     }
                { if let SettingsRoute::Profile = route {
                    html!{
                        <div class="settings-body-header">
                            <h1>{ "Profile" }</h1>
                        </div>
                    }
                } else if let SettingsRoute::Tickets = route {
                    html!{
                        <div class="settings-body-header">
                            <h1>{ "Tickets" }</h1>
                        </div>
                    }
                } else if let SettingsRoute::Account { user_id } = route {
                    html!{
                        <div class="settings-body-header">
                            <h1>{ "Account" }</h1>
                        </div>
                    }
                } else {
                    html!{}
                }}
            </div>
        </div>
    }

}
