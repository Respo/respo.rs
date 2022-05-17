use std::{collections::HashMap, rc::Rc, sync::RwLock};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{div, div0, render_node, span0, DispatchFn, RespoCssStyle, RespoEventHandler, RespoNode};

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
    Box::new(move || -> Result<RespoNode<ActionOp>, String> {
      Ok(
        div0()
          .add_children([
            span0()
              .add_attrs([("innerText".to_owned(), "demo inc".to_owned())])
              .add_event([(
                "click",
                RespoEventHandler(Rc::new(move |e, dispatch| -> Result<(), String> {
                  log_1(&format!("click {:?}", e).into());
                  dispatch.run(ActionOp::Increment)?;
                  Ok(())
                })),
              )])
              .to_owned(),
            span0()
              .add_attrs([("innerText".to_owned(), "demo dec".to_owned())])
              .add_event([(
                "click".to_owned(),
                RespoEventHandler(Rc::new(move |e, dispatch| -> Result<(), String> {
                  log_1(&format!("click {:?}", e).into());
                  dispatch.run(ActionOp::Decrement)?;
                  Ok(())
                })),
              )])
              .to_owned(),
          ])
          .to_owned(),
      )
    }),
    DispatchFn(Rc::new(dispatch_action)),
  )
  .unwrap();

  JsValue::NULL
}
