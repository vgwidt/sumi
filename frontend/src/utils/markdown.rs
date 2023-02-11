use pulldown_cmark::{html::push_html, Options, Parser};
use web_sys::Node;
use yew::{virtual_dom::VNode, Html};

pub fn markdown_to_html(body: &str) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(body, options);

    let mut html_text = String::new();
    push_html(&mut html_text, parser);

    //set all checkboxes to disabled
    let html_text = html_text.replace(
        "<input type=\"checkbox\"",
        "<input type=\"checkbox\" disabled",
    );

    //Sanitize
    //allow input type of checkbox
    let html_text = ammonia::Builder::default()
        .add_tags(std::iter::once("input"))
        .add_tag_attribute_values("input", "type", std::iter::once("checkbox"))
        .add_tag_attribute_values("input", "checked", std::iter::once(""))
        .add_tag_attribute_values("input", "disabled", std::iter::once(""))
        .clean(&*html_text)
        .to_string();

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();
    div.set_inner_html(&html_text);
    let node = Node::from(div);
    VNode::VRef(node)
}
