use pulldown_cmark::Parser;
use ammonia::clean;
use web_sys::Node;
use yew::{virtual_dom::VNode, Html};

pub fn markdown_to_html(body: &str) -> Html {
    let parser = Parser::new(body);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);

    //Sanitize
    let html_text = clean(&*html_text);

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
