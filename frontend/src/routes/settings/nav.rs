use yew::prelude::*;
use yew_router::prelude::Link;

use crate::hooks::{use_language_context, use_user_context};
use crate::routes::SettingsRoute;

/// Update user settings
#[function_component(SettingsNav)]
pub fn settings_nav() -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();

    html!{
        <div class="settings">
            <div class="settings-nav">
                <div class="settings-nav-header">
                    <h1>{ "Settings" }</h1>
                </div>
                <div class="settings-nav-body">
                    <ul>
                        <li>
                            <Link<SettingsRoute> to={SettingsRoute::Profile} classes="nav-link">
                                { language.get("Profile") }
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
        </div>
    }
}