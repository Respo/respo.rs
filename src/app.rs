extern crate console_error_panic_hook;

mod counter;
mod data_types;
mod panel;
mod task;
mod todolist;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::{panic, vec};

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use data_types::*;

use crate::respo::{
  button, div, render_node, span, util::query_select_node, CssColor, CssRule, DispatchFn, RespoEvent, RespoEventHandler, RespoNode,
  StatesTree,
};
use crate::respo::{declare_static_style, RespoEffect, RespoEffectHandler};

use self::counter::comp_counter;
use self::panel::comp_panel;
use self::todolist::comp_todolist;

#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let mount_target = query_select_node(".app").expect("found mount target");

  // need to push store inside function to keep all in one thread
  let global_store = Rc::new(RefCell::new(Store {
    counted: 0,
    states: StatesTree::default(),
    tasks: vec![],
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
      let states = store.states.clone();

      Ok(
        div()
          .add_children([
            comp_counter(&states.pick("counter"), store.counted),
            comp_panel(&states.pick("panel"))?,
            comp_todolist(&states.pick("todolist"), &vec![])?,
          ])
          .to_owned(),
      )
    }),
    DispatchFn(Rc::new(dispatch_action)),
  )
  .expect("rendering node");

  JsValue::NULL
}
