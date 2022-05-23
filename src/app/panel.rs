use std::{fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console::log_1;

use crate::{
  app::data_types::ActionOp,
  button,
  respo::{div, input, span, util, RespoEffect, RespoEffectHandler, RespoEvent, RespoNode, StatesTree},
  space,
  ui::{ui_button, ui_input},
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PanelState {
  content: String,
}

pub fn comp_panel(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let cursor2 = cursor.clone();
  let cursor3 = cursor.clone();
  let state = match &states.data {
    Some(v) => serde_json::from_value(v.to_owned()).expect("to panel state"),
    None => PanelState::default(),
  };
  let state2 = state.clone();

  Ok(RespoNode::Component(
    "panel".to_owned(),
    vec![RespoEffect {
      args: vec![],
      handler: RespoEffectHandler(Rc::new(move |args, action_type, el| -> Result<(), String> {
        util::log!("panel mounted");
        Ok(())
      })),
    }],
    Box::new(
      div()
        .add_children([
          input()
            .class(ui_input())
            .insert_attr("placeholder", "some content...")
            .insert_attr("value", state.content.to_owned())
            .on_input(Rc::new(move |e, dispatch| -> _ {
              util::log!("input event: {:?}", e);
              if let RespoEvent::Input { value, .. } = e {
                dispatch.run(ActionOp::StatesChange(
                  cursor.to_owned(),
                  Some(serde_json::to_value(PanelState { content: value }).expect("to json")),
                ))?;
              }
              Ok(())
            }))
            .to_owned(),
          space(Some(8), None),
          button()
            .class(ui_button())
            .insert_attr("innerText", "add")
            .on_click(Rc::new(move |e, dispatch| -> Result<(), String> {
              util::log!("add button {:?}", e);
              dispatch.run(ActionOp::AddTask(Uuid::new_v4().to_string(), state2.content.to_owned()))?;
              dispatch.run(ActionOp::StatesChange(
                cursor3.clone(),
                Some(serde_json::to_value(PanelState { content: "".to_owned() }).expect("to json")),
              ))?;
              Ok(())
            }))
            .to_owned(),
          span()
            .add_attrs([("innerText", format!("got panel state: {:?}", state.to_owned()))])
            .to_owned(),
        ])
        .to_owned(),
    ),
  ))
}
