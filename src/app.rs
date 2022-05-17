extern crate console_error_panic_hook;

use std::panic;
use std::{rc::Rc, sync::RwLock};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{div0, render_node, span0, DispatchFn, RespoEventHandler, RespoNode};

lazy_static::lazy_static! {
  static ref GLOBAL_STORE: RwLock<Store> = RwLock::new(Store::default());
}

#[derive(Clone, Debug, Default)]
struct Store {
  counted: i32,
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
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  render_node(
    ".app",
    Box::new(move || -> Result<RespoNode<ActionOp>, String> {
      let store = GLOBAL_STORE.read().expect("to render");
      Ok(
        div0()
          .add_children([
            span0()
              .add_attrs([("innerText", format!("value is: {}", store.counted))])
              .to_owned(),
            span0()
              .add_attrs([("innerText", "demo inc")])
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
              .add_attrs([("innerText", "demo dec")])
              .add_event([(
                "click",
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
