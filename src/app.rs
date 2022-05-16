use std::{collections::HashMap, rc::Rc};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{div, render_node, span, RespoCssStyle, RespoEventHandler};

#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let mount_target = document.query_selector(".app").expect("should have a .app").unwrap();

  let tree = div(
    HashMap::new(),
    RespoCssStyle(HashMap::new()),
    HashMap::new(),
    vec![
      div(HashMap::new(), RespoCssStyle(HashMap::new()), HashMap::new(), vec![]),
      span(
        HashMap::from_iter([("innerText".to_owned(), "a demo".to_owned())]),
        RespoCssStyle(HashMap::new()),
        HashMap::from_iter([(
          "click".to_owned(),
          RespoEventHandler(Rc::new(move || {
            log_1(&"click".into());
            Ok(())
          })),
        )]),
        vec![],
      ),
    ],
  );

  render_node(&mount_target, &tree).unwrap();

  JsValue::NULL
}
