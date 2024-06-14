use std::fmt::Debug;

use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::ActionOp;

use respo::{
  button, div, input, space, span,
  ui::{ui_button, ui_input},
  util, DispatchFn, RespoComponent, RespoEffect, RespoEffectBox, RespoEvent, RespoNode,
};

use respo::states_tree::{RespoState, RespoStatesTree};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, RespoState)]
struct PanelState {
  content: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PanelMount {}

impl RespoEffect for PanelMount {
  fn as_any(&self) -> &dyn std::any::Any {
    self
  }
  fn do_eq(&self, rhs: &dyn RespoEffect) -> Option<bool> {
    rhs.as_any().downcast_ref::<Self>().map(|x| x == self)
  }

  fn updated(&self, _el: &web_sys::Node) -> Result<(), String> {
    respo::util::log!("panel updated");
    Ok(())
  }

  fn mounted(&self, _el: &web_sys::Node) -> Result<(), String> {
    respo::util::log!("panel mounted");
    Ok(())
  }
}

pub fn comp_panel(states: &RespoStatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = states.cast_branch::<PanelState>()?;

  let on_input = {
    let cursor = cursor.to_owned();
    move |e, dispatch: DispatchFn<_>| -> _ {
      util::log!("input event: {:?}", e);
      if let RespoEvent::Input { value, .. } = e {
        dispatch.run_state(&cursor, PanelState { content: value })?;
      }
      Ok(())
    }
  };

  let on_submit = {
    let state = state.to_owned();
    let cursor = cursor.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("add button {:?}", e);
      dispatch.run(ActionOp::AddTask(Uuid::new_v4().to_string(), state.content.to_owned()))?;
      dispatch.run_state(&cursor, PanelState { content: "".to_owned() })?;
      Ok(())
    }
  };

  let mount_effect = RespoEffectBox::new(PanelMount::default());

  Ok(
    RespoComponent::named(
      "panel",
      div().elements([
        input()
          .class(ui_input())
          .attribute("placeholder", "some content...")
          .attribute("value", state.content.to_owned())
          .on_input(on_input),
        space(Some(8), None),
        button().class(ui_button()).inner_text("add").on_click(on_submit),
        span().inner_text(format!("got panel state: {:?}", state)),
      ]),
    )
    .stable_effect(mount_effect)
    .to_node(),
  )
}
