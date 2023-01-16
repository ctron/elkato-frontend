#![recursion_limit = "1024"]

mod app;
mod pages;
mod state;

pub(crate) mod utils;

use wasm_bindgen::prelude::*;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = log::Level::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL).module_prefix("elkato_frontend"));
    yew::Renderer::<app::Main>::new().render();
    Ok(())
}
