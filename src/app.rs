extern crate console_error_panic_hook;

use std::cell::RefCell;
use std::rc::Rc;
use std::{any::Any, fmt::Debug};
use std::{panic, vec};

use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

use crate::respo::{
  button, div, render_node, span, util::query_select_node, CssColor, CssRule, DispatchFn, LocalState, RespoCacheable, RespoEffectArg,
  RespoEvent, RespoEventHandler, RespoNode, StatesTree,
};
use crate::respo::{RespoEffect, RespoEffectHandler};

#[derive(Debug)]
struct Store {
  counted: i32,
  tasks: Vec<Task>,
  states: StatesTree,
}

#[derive(Debug, Clone)]
struct Task {
  done: bool,
  content: String,
  time: f32,
}

impl RespoCacheable for Task {
  fn as_any(&self) -> &dyn Any {
    self
  }
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

impl RespoCacheable for MainState {
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
                      if let RespoEvent::Click { original_event, .. } = e {
                        original_event.prevent_default();
                      }

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
                      log_1(&format!("click {:?}", e,).into());
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

fn comp_panel<T>(states: &StatesTree) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  Ok(RespoNode::Component(
    "panel".to_owned(),
    vec![RespoEffect {
      args: vec![],
      handler: RespoEffectHandler(Rc::new(move |args, action_type, el| -> Result<(), String> {
        log_1(&"TODO".into());
        Ok(())
      })),
    }],
    Box::new(
      div()
        .add_children([span().add_attrs([("innerText", String::from("TODO panel"))]).to_owned()])
        .to_owned(),
    ),
  ))
}

fn comp_todolist<T>(states: &StatesTree, tasks: &Vec<Task>) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  Ok(
    div()
      .add_children([
        span().add_attrs([("innerText", format!("TODO {:?}", tasks))]).to_owned(),
        comp_task(
          &states.pick("task"),
          &Task {
            done: false,
            content: String::from("task 1"),
            time: 0.0,
          },
        )?,
      ])
      .to_owned(),
  )
}

fn comp_task<T>(states: &StatesTree, task: &Task) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  let arg: Rc<dyn RespoCacheable> = Rc::new(task.to_owned());

  Ok(RespoNode::Component(
    "tasks".to_owned(),
    vec![RespoEffect {
      args: vec![RespoEffectArg(arg)],
      handler: RespoEffectHandler(Rc::new(move |args, effect_type, el| -> Result<(), String> {
        // TODO
        Ok(())
      })),
    }],
    Box::new(
      div()
        .add_children([span().add_attrs([("innerText", format!("TODO {:?}", task))]).to_owned()])
        .to_owned(),
    ),
  ))
}
