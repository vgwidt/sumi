mod account;
mod nav;
mod ticket_fields;

use stylist::style;
use yew::prelude::*;
use yew_router::prelude::use_route;

use crate::contexts::theme;
use crate::routes::SettingsRoute;
use crate::routes::settings::account::AccountSettings;
use crate::routes::settings::nav::SettingsNav;
use crate::routes::settings::ticket_fields::TicketFields;

use super::AppRoute;

/// Update user settings
#[function_component(Settings)]
pub fn settings() -> Html {
    let theme = theme::use_theme();

    let route = match use_route::<SettingsRoute>() {
        Some(route) => route,
        None => SettingsRoute::Profile,
    };

    let style = style! {
        r#"
        .settings {
            display: flex;
            flex-direction: row;
            height: 100%;
            width: 100%;
        }
        .settings-nav {
            flex-grow: 1;
            max-width: 200px;
            overflow: auto;
            height: 100%;
            border-right: 1px solid #777;
            width: 30%;
        }
        .settings-nav ul {
            list-style: none;
            padding: 2px 0 0px 20px;
        }
        .settings-nav li {
            list-style: none;
            padding: 2px 0 2px 0px;
            font-size: 1.2em;
        }     
        .nav-link {
            padding: 2px 8px;
            text-decoration: none;
            border: 1px solid transparent;
          }
          .selected {
            border: 1px solid ${border};
            border-radius: 8px;
            background: ${bg};
        }
        .settings-body {
            width: 100%;
            min-width: 300px;
            height: 100%;
            overflow: auto;
            margin-left: 32px;
            margin-right: 32px;
        }
        "#,
        bg = theme.background.clone(),
        border = theme.border.clone(),
    }
    .expect("Failed to parse style");
    
    //Settings will display the nav bar, then depending on the route, will display the appropriate page (Account settings, ticket settings, etc)
    html!{
        //Match SettingsRoute to display the appropriate page
        <div class={style}>
            <div class="settings">
                <SettingsNav />
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
                                <h1>{ "Ticket Settings" }</h1>
                                <TicketFields />
                            </div>
                        }
                    } else if let SettingsRoute::Account { user_id } = route {
                        html!{
                            <div class="settings-body-header">
                                <AccountSettings user_id={Some(user_id.clone())}/>
                            </div>
                        }
                    } else {
                        html!{}
                    }}
                </div>
            </div>
        </div>
    }

}
