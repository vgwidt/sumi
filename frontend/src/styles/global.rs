use stylist::yew::styled_component;
use stylist::yew::Global;
use yew::html;
use yew::Html;

use crate::contexts::theme::use_theme;

// GlobalStyle as a component
// It may be better to just create a CONST string and use Global in app.rs
#[styled_component(GlobalStyle)]
pub fn global_style() -> Html {
    let theme = use_theme();

    html! {
        <Global css={css!(
            r#"
            body {
                background-color: ${bg};
                color: ${text};
                font-family: 'Nunito Sans', sans-serif;
                margin-left: 200px;
                font-size: 14px;  
                padding: 16px;
            }
            a {
                color: ${text};
            }
            .logo {
                width: 50%;
                filter: invert(${logo_inversion});
            }
            a:hover {
                color: ${link};
            }
            a:visited {
                color: ${text};
            }
            input {
                background-color: ${edit_bg};
                border-radius: 4px;
                border: none;
                color: ${text};
                display: block;
                padding: 8px;
                margin-top: 4px;
                margin-bottom: 8px;
            }
            textarea {
                background-color: ${edit_bg};
                border-radius: 4px;
                border: none;
                color: ${text};
                display: block;
                padding: 8px;
                margin: 4px 0px;
            }
            label {
                display: inline-block;
                margin-right: 8px;
                text-align: right;
            }
            .error {
                color: #ed3434;
                text-align: center;
             }
             .btn {
                background-color: #39398e;
                color: rgb(233, 231, 231);
                border-radius: 4px;
                padding: 6px;
                margin: 4px;
                display: inline-flex;
                text-decoration: none;
                border: none;
                justify-content: center;
                min-width: 60px;
              }
              a.btn {
                color: rgb(233, 231, 231);
              }
              .btn:hover {
                  background-color: #343435;
                  cursor: pointer;
                }
            fieldset {
                border: none;
            }
            "#,
            bg = theme.background_color.clone(),
            text = theme.font_color.clone(),
            link = theme.link_color.clone(),
            logo_inversion = theme.logo_inversion.clone(),
            edit_bg = theme.edit_background_color.clone()
        )} />
    }
}
