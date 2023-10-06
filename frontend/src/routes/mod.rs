pub mod home;
pub mod login;
pub mod new_user;
pub mod settings;
pub mod ticket;
pub mod ticket_editor;
pub mod users;
pub mod wiki;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use login::Login;
use new_user::NewUser;
use settings::Settings;
use ticket::Ticket;
use ticket_editor::TicketEditor;
use users::Users;
use wiki::Wiki;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/editor/:ticket_id")]
    Editor { ticket_id: i32 },
    #[at("/editor")]
    EditorCreate,
    #[at("/ticket/:ticket_id")]
    Ticket { ticket_id: i32 },
    #[at("/wiki")]
    WikiHome,
    #[at("/wiki/:document_id")]
    WikiDoc { document_id: uuid::Uuid },
    #[at("/settings")]
    SettingsRoot,
    #[at("/settings/*")]
    Settings,
    #[at("/users")]
    Users,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum SettingsRoute {
    #[at("/settings")]
    Profile,
    #[at("/settings/account/:user_id")]
    Account { user_id: uuid::Uuid },
    #[at("/settings/tickets")]
    Tickets,
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Login => html! {<Login />},
        AppRoute::Register => html! {<NewUser />},
        AppRoute::Home => html! {<Home />},
        AppRoute::Editor { ticket_id } => {
            html! {<TicketEditor ticket_id={Some(ticket_id.clone())}/>}
        }
        AppRoute::EditorCreate => html! {<TicketEditor ticket_id={None} />},
        AppRoute::Ticket { ticket_id } => html! {<Ticket ticket_id={ticket_id.clone()} />},
        // AppRoute::Settings => html! {<Settings />},
        // AppRoute::SettingsOther { user_id } => html! {<Settings user_id={user_id.clone()}/>},
        AppRoute::SettingsRoot | AppRoute::Settings => {
            html! { <Switch<SettingsRoute> render={switch_settings} /> }
        }
        AppRoute::Users => html! {<Users />},
        AppRoute::NotFound => html! { "Page not found" },
        AppRoute::WikiHome => html! {<Wiki document_id={None}/>},
        AppRoute::WikiDoc { document_id } => html!(<Wiki document_id={Some(document_id.clone())}/>),
    }
}

pub fn switch_settings(route: SettingsRoute) -> Html {
    match route {
        SettingsRoute::Profile => html! {<Settings />},
        SettingsRoute::Account { user_id: _ } => html! {<Settings />},
        SettingsRoute::Tickets => html! {<Settings />},
        SettingsRoute::NotFound => html! {<Redirect<AppRoute> to={AppRoute::NotFound}/>},
    }
}
