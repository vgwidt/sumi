use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Navigation;
use crate::contexts::language::LanguageProvider;
use crate::contexts::theme::ThemeProvider;
use crate::contexts::user::UserContextProvider;
use crate::routes::{switch, AppRoute};
use crate::styles::global::GlobalStyle;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
        <GlobalStyle />
            <UserContextProvider>
                <BrowserRouter>
                    <LanguageProvider>
                        <Navigation />
                        <Switch<AppRoute> render={switch} />
                    </LanguageProvider>
              </BrowserRouter>
          </UserContextProvider>
        </ThemeProvider>
    }
}
