use std::{cell::RefCell, rc::Rc};

use crate::render_element;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

struct A {
  callback: Box<dyn FnMut() -> f32>,
}

#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  let b = Rc::new(RefCell::new(1.0));
  let mut a = A {
    callback: Box::new(move || {
      let mut bi = (*b).borrow_mut();
      let ret = *bi;
      *bi += 1.;
      ret
    }),
  };

  log_1(&(a.callback)().to_string().into());

  log_1(&(a.callback)().to_string().into());

  log_1(&(a.callback)().to_string().into());

  log_1(&(a.callback)().to_string().into());

  log_1(&(a.callback)().to_string().into());

  render_element();

  JsValue::NULL
}
