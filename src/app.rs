use std::{collections::HashMap, rc::Rc, sync::RwLock};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{div, render_node, span, DispatchFn, RespoCssStyle, RespoEventHandler};

lazy_static::lazy_static! {
  static ref GLOBAL_STORE: RwLock<Store> = RwLock::new(Store::default());
}

#[derive(Clone, Debug, Default)]
struct Store {
  counted: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ActionOp {
  Increment,
  Decrement,
}

fn dispatch_action(op: ActionOp) -> Result<(), String> {
  log_1(&format!("action {:?}", op).into());
  let mut store = GLOBAL_STORE.write().expect("to dispatch action");
  match op {
    ActionOp::Increment => {
      store.counted += 1;
    }
    ActionOp::Decrement => {
      store.counted -= 1;
    }
  }
  Ok(())
}

#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let mount_target = document.query_selector(".app").expect("should have a .app").unwrap();

  render_node(
    &mount_target,
    Box::new(move || {
      div(
        HashMap::new(),
        RespoCssStyle(HashMap::new()),
        HashMap::new(),
        vec![
          div(HashMap::new(), RespoCssStyle(HashMap::new()), HashMap::new(), vec![]),
          span(
            HashMap::from_iter([("innerText".to_owned(), "demo inc".to_owned())]),
            RespoCssStyle(HashMap::new()),
            HashMap::from_iter([(
              "click".to_owned(),
              RespoEventHandler(Rc::new(move |e, dispatch| -> Result<(), String> {
                log_1(&"click".into());
                (*dispatch.0)(ActionOp::Increment)?;
                Ok(())
              })),
            )]),
            vec![],
          ),
          span(
            HashMap::from_iter([("innerText".to_owned(), "demo dec".to_owned())]),
            RespoCssStyle(HashMap::new()),
            HashMap::from_iter([(
              "click".to_owned(),
              RespoEventHandler(Rc::new(move |e, dispatch| -> Result<(), String> {
                log_1(&"click".into());
                (*dispatch.0)(ActionOp::Decrement)?;
                Ok(())
              })),
            )]),
            vec![],
          ),
        ],
      )
    }),
    DispatchFn(Rc::new(dispatch_action)),
  )
  .unwrap();

  JsValue::NULL
}
