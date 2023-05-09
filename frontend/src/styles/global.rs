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
                background-color: ${input_bg};
                border-radius: 4px;
                border: 1px solid ${border};
                color: ${text};
                padding: 8px;
                margin-top: 4px;
                margin-bottom: 8px;
                box-sizing: border-box;
            }
            textarea {
                background-color: ${input_bg};
                border-radius: 4px;
                border: 1px solid ${border};
                color: ${text};
                display: block;
                padding: 8px;
                margin: 4px 0px;
                box-sizing: border-box;
                resize: vertical;
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
                background-color: #35358a;
                color: #e9e7e7;
                border-radius: 4px;
                padding: 6px;
                margin: 4px;
                display: inline-flex;
                border: none;
                justify-content: center;
                min-width: 32px;
                vertical-align: middle;
            }
            .btn:hover {
                background-color: #5c5c8d;
                cursor: pointer;
            }
            fieldset {
                border: none;
            }
            code {
                background: ${code_bg};
                padding: 4px;
                border-radius: 4px;
            }
            pre {
                width: auto;
                overflow-x: auto;
                background: ${code_bg};
                padding: 10px;
                border-radius: 4px;
            }
            pre code {
                background: transparent;
                padding: 0px;
                border-radius: 0px;
            }
            select {
                background: ${input_bg};
                border: 1px solid ${border};
                border-radius: 4px;
                padding: 4px;
                color: ${text};
            }
            .btn-action {
                padding: 4px;
                background-color: ${bg};
                color: ${text};
                border: 1px solid transparent;
                border-radius: 2px;
                margin-left: auto;
                margin-right: 0px;
                display: block;
            }
            .btn-action:hover {
                border: 1px solid ${border};
            }
            .btn-action-active {
                border: 1px solid ${border};
            }
            .dropdown {
                position: relative;
                float: right;
                display: inline-block;
            }
            .dropdown-content {
                box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
                transition: 0.1s;
            }
            .dropdown-content .btn {
                background-color: inherit;
                color: ${text};
                border: none;
                margin: 0px;
                border-radius: 0px;
                font-size: 14px;
                min-width: 120px;
                background-color: ${menu_background};
                padding: 8px 0px;
            }
            .dropdown-content .btn:hover {
                background-color: #35358a;
                color: #e9e7e7;
            }
            .page-btn {
                border: none;
                border-radius: 4px;
                color: ${text};
                background-color: transparent;
                cursor: pointer;
              }
            .edit-icon {
                display: inline-block;
                transform: rotateZ(100deg);
                font-size: 1.4em;
            }
            .add-icon {
                display: inline-block;
                font-size: 1.4em;
                font-weight: bold;
            }
            "#,
            bg = theme.background.clone(),
            text = theme.font_color.clone(),
            link = theme.link_color.clone(),
            logo_inversion = theme.logo_inversion.clone(),
            input_bg = theme.input_background.clone(),
            code_bg = theme.code_background.clone(),
            border = theme.border.clone(),
            menu_background = theme.menu_background.clone(),
        )} />
    }
}
