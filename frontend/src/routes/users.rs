use yew::{prelude::*, suspense::use_future};
use yew_router::prelude::{Link, Redirect};

use crate::{hooks::use_user_context, services::users::get_users};

use super::AppRoute;

#[function_component(Users)]
pub fn users() -> Html {
    let user_ctx = use_user_context();
    let users = use_future(|| async { get_users().await.unwrap_or_default() });

    let user_list = match users {
        Ok(users) => users.clone(),
        Err(_) => vec![],
    };

    if user_ctx.is_authenticated() {
        html! {
            <div>
                <h1>{ "Users" }</h1>
                <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                    { "Create new user" }
                </Link<AppRoute>>
                <table>
                    <thead>
                        <tr>
                            <th>{ "Username" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for user_list.iter().map(|user| {
                                html! {
                                    <tr>
                                    <Link<AppRoute> to={AppRoute::SettingsOther { user_id: user.user_id.clone() }} classes="nav-link">
                                        <td>{ &user.username }</td>
                                    </Link<AppRoute>>
                                    </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
            </div>
        }
    } else {
        html! {
            <Redirect<AppRoute> to={AppRoute::Login} />
        }
    }
}
