extern crate console_error_panic_hook;

mod counter;
mod data_types;
mod panel;
mod task;
mod todolist;

use std::cell::RefCell;
use std::rc::Rc;
use std::{panic, vec};

use wasm_bindgen::prelude::*;

use crate::respo::{div, render_node, util::query_select_node, DispatchFn, RespoNode, StatesTree};
use crate::ui::ui_global;

use self::counter::comp_counter;
use self::data_types::*;
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
    // util::log!("action {:?}", op);
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
          .class(ui_global())
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
