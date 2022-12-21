use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::select_theme::ThemeToggle;
use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;

#[styled_component(Navigation)]
pub fn navigation() -> Html {
    let user_ctx = use_user_context();
    let language = use_language_context();
    let theme = use_theme();

    let style = style!(
        r#"
        .sidenav {
            height: 100%;
            width: 200px;
            position: fixed;
            z-index: 1;
            top: 0;
            left: 0;
            overflow-x: hidden;
            padding-top: 20px;
            background-color: ${bg};
            color: ${text};
            display:flex; 
            flex-direction:column;
        }
        .nav-main {
            flex:1;
        }
        .nav-footer {
            min-height: 80px;
        }
        .logo {
            margin: 0 auto 10px;
            padding: 2px;
            display: flex;
            justify-content: center;
            text-decoration: none;
          }
          .nav-user {
            font-size: 16px;
            display: flex;
            justify-content: center;
            margin-bottom: 20px;
          }
          .nav-headers {
          }
          .sidenav a {
            display: block;
          }
          .nav-link {
            font-size: 20px;
            margin-left: 8px;
            margin-right: 8px;
            padding-top: 8px;
            padding-bottom: 8px;
            padding-left: 16px;
          }
          .sidenav a:hover {
            color: #f1f1f1;
          }
          .nav-theme-toggle {
            font-size: 16px;
            padding-bottom: 4px;
            display: flex;
            justify-content: center;
          }
          .footer {
            display: flex;
            justify-content: center;
          }
        "#,
        bg = theme.secondary_background.clone(),
        text = theme.font_color.clone()
    )
    .expect("Failed to parse style");

    if user_ctx.is_authenticated() {
        html! {
            <div class={style}>
                <nav class="sidenav">
                <div class="nav-main">
                    <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                        <img src="./img/Enso.svg" alt="Enso Logo" class="logo" />
                    </Link<AppRoute>>
                    <div class="nav-user" >
                        <Link<AppRoute> to={AppRoute::Settings}>
                            { user_ctx.display_name.clone() }
                        </Link<AppRoute>>
                    </div>
                    <div class="nav-headers">
                        <Link<AppRoute> to={AppRoute::Home} classes="nav-link">
                            { language.get("Tickets") }
                        </Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::WikiHome} classes="nav-link">
                            { language.get("Wiki") }
                        </Link<AppRoute>>
                        <Link<AppRoute> to={AppRoute::Users} classes="nav-link">
                            { language.get("Users") }
                        </Link<AppRoute>>
                            //{ "Contacts" }
                            //{ "Assets" }
                            //{ "Reports" }
                    </div>
                    </div>
                    <div class="nav-footer">
                        <div class="nav-theme-toggle">
                          <ThemeToggle />
                        </div>
                        <footer>
                        <div class="container">
                            <span class="footer">
                                <a href="https://github.com/vgwidt/sumi"> { "Sumi Ticketing System" } </a>
                            </span>
                        </div>
                    </footer>
                    </div>
                </nav>
            </div>
        }
    } else {
        html! {}
    }
}
