pub mod app;
pub mod components;
pub mod contexts;
pub mod hooks;
pub mod routes;
pub mod services;
pub mod styles;
pub mod types;
pub mod utils;

use wasm_bindgen::prelude::*;

use app::App;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
    Ok(())
}
