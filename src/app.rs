extern crate console_error_panic_hook;

use std::any::Any;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{
  button, div, query_select_node, render_node, span, CssColor, CssRule, DispatchFn, LocalState, LocalStateAbstract, RespoEventHandler,
  RespoNode, StatesTree,
};

#[derive(Debug)]
struct Store {
  counted: i32,
  states: StatesTree,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ActionOp {
  Increment,
  Decrement,
  StatesChange(Vec<String>, LocalState),
}

#[derive(Debug, Clone, Default)]
struct MainState {
  counted: i32,
}

impl LocalStateAbstract for MainState {
  fn as_any(&self) -> &dyn Any {
    self
  }
}

#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let mount_target = query_select_node(".app").expect("found mount target");

  // need to push store inside function to keep all in one thread
  let global_store = Rc::new(RefCell::new(Store {
    counted: 0,
    states: StatesTree::default(),
  }));

  let store_to_action = global_store.clone();
  let dispatch_action = move |op: ActionOp| -> Result<(), String> {
    // log_1(&format!("action {:?}", op).into());
    let mut store = store_to_action.borrow_mut();
    match op {
      ActionOp::Increment => {
        store.counted += 1;
      }
      ActionOp::Decrement => {
        store.counted -= 1;
      }
      ActionOp::StatesChange(path, new_state) => {
        store.states = store.states.set_in(&path, new_state);
      }
    }
    Ok(())
  };

  render_node(
    mount_target,
    Box::new(move || -> Result<RespoNode<ActionOp>, String> {
      let store = global_store.borrow();
      let states = store.states.to_owned();
      let cursor = states.path();

      let state: MainState = states.load().ref_into::<MainState>().map(ToOwned::to_owned).unwrap_or_default();

      Ok(
        div()
          .add_children([
            div()
              .add_children([
                button()
                  .add_attrs([("innerText", "demo inc"), ("class", "my-button")])
                  .add_style([CssRule::Margin(4.)])
                  .add_event([(
                    "click",
                    RespoEventHandler(Rc::new(move |e, dispatch| -> Result<(), String> {
                      log_1(&format!("click {:?}", e).into());
                      dispatch.run(ActionOp::Increment)?;
                      dispatch.run(ActionOp::StatesChange(
                        cursor.to_owned(),
                        LocalState::ref_from(Some(&MainState {
                          counted: state.counted + 2,
                        })),
                      ))?;
                      Ok(())
                    })),
                  )])
                  .to_owned(),
                button()
                  .add_attrs([("innerText", "demo dec"), ("class", "my-button")])
                  .add_style([CssRule::Margin(4.)])
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
            div()
              .add_children([span()
                .add_attrs([("innerText", format!("value is: {}", store.counted))])
                .add_style([
                  CssRule::Color(CssColor::Blue),
                  CssRule::FontFamily("Menlo".to_owned()),
                  CssRule::FontSize(10.0 + store.counted as f32),
                ])
                .to_owned()])
              .to_owned(),
            div()
              .add_children([span()
                .add_attrs([("innerText", format!("local state: {}", state.counted))])
                .to_owned()])
              .to_owned(),
          ])
          .to_owned(),
      )
    }),
    DispatchFn(Rc::new(dispatch_action)),
  )
  .expect("rendering node");

  JsValue::NULL
}
