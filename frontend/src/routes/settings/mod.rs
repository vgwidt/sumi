mod account;
mod nav;

use yew::prelude::*;
use yew_router::prelude::use_route;

use crate::routes::SettingsRoute;
use crate::routes::settings::account::AccountSettings;
use crate::routes::settings::nav::SettingsNav;

use super::AppRoute;

/// Update user settings
#[function_component(Settings)]
pub fn settings() -> Html {

    let route = match use_route::<SettingsRoute>() {
        Some(route) => route,
        None => SettingsRoute::Profile,
    };
    
    //Settings will display the nav bar, then depending on the route, will display the appropriate page (Account settings, ticket settings, etc)
    html!{
        //Match SettingsRoute to display the appropriate page
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
                            <h1>{ "Tickets" }</h1>
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
    }

}
