use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console::{log_1, log_2};

#[wasm_bindgen(js_name = renderElement)]
pub fn render_element() {
  log_1(&"Respo TODOs".into());
}
