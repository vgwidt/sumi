use stylist::{style, yew::styled_component};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::select_theme::ThemeToggle;
use crate::contexts::theme::use_theme;
use crate::hooks::use_language_context;
use crate::hooks::use_user_context;
use crate::routes::AppRoute;

#[styled_component(Navigation)]
pub fn navigation() -> HtmlResult {
    let user_ctx = use_user_context();
    let language = use_language_context()?;
    let theme = use_theme();

    //when use_route changes, we change the active tabs style to selected
    let route = match use_route::<AppRoute>() {
        Some(route) => route,
        None => AppRoute::Home,
    };

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
            background-color: ${nav_bg};
            color: ${text};
            display:flex; 
            flex-direction:column;
            box-shadow: 5px 0 10px rgba(0, 0, 0, 0.2);
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
          .sidenav .nav-link:hover {
            border: 1px solid ${border};
            border-radius: 8px;
          }
          .nav-link {
            font-size: 20px;
            margin-left: 8px;
            margin-right: 8px;
            padding-top: 8px;
            padding-bottom: 8px;
            padding-left: 16px;
            text-decoration: none;
            border: 1px solid transparent;
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
          .selected {
            border: 1px solid ${border};
            border-radius: 8px;
            background: ${bg};
        }
        "#,
        bg = theme.background.clone(),
        nav_bg = theme.secondary_background.clone(),
        text = theme.font_color.clone(),
        border = theme.border.clone(),
    )
    .expect("Failed to parse style");

    if user_ctx.is_authenticated() {
        Ok(html! {
            <div class={style}>
                <nav class="sidenav">
                    <div class="nav-main">
                        <Link<AppRoute> to={AppRoute::Home} classes="navbar-brand">
                            <img src="./img/Enso.svg" alt="Enso Logo" class="logo" />
                        </Link<AppRoute>>
                        <div class="nav-user">
                            <Link<AppRoute> to={AppRoute::Settings}>
                                { user_ctx.display_name.clone() }
                            </Link<AppRoute>>
                        </div>
                        <div class="nav-headers">
                            <Link<AppRoute> to={AppRoute::Home} classes={
                                if route == AppRoute::Home {
                                "selected nav-link"
                                } else {
                                "nav-link"
                                }
                                }>
                                { language.get("Tickets") }
                            </Link<AppRoute>>
                            <Link<AppRoute> to={AppRoute::WikiHome} classes={
                                if route == AppRoute::WikiHome {
                                "selected nav-link"
                                } else {
                                "nav-link"
                                }
                                }>
                                { language.get("Wiki") }
                            </Link<AppRoute>>
                            <Link<AppRoute> to={AppRoute::Users} classes={
                                if route == AppRoute::Users {
                                "selected nav-link"
                                } else {
                                "nav-link"
                                }
                                }>
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
        })
    } else {
        Ok(html! {})
    }
}
