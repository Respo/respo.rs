use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console::log_1;

use crate::{
  app::data_types::ActionOp,
  button,
  respo::{div, input, span, util, RespoEffect, RespoEvent, RespoNode, StatesTree},
  space,
  ui::{ui_button, ui_input},
  DispatchFn,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PanelState {
  content: String,
}

pub fn comp_panel(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let cursor2 = cursor.clone();
  let cursor3 = cursor.clone();
  let state: PanelState = states.data.cast_or_default()?;
  let state2 = state.clone();

  let on_input = move |e, dispatch: DispatchFn<_>| -> _ {
    util::log!("input event: {:?}", e);
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run_state(&cursor2, PanelState { content: value })?;
    }
    Ok(())
  };

  let on_submit = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("add button {:?}", e);
    dispatch.run(ActionOp::AddTask(Uuid::new_v4().to_string(), state2.content.to_owned()))?;
    dispatch.run_state(&cursor3, PanelState { content: "".to_owned() })?;
    Ok(())
  };

  Ok(RespoNode::Component(
    "panel".to_owned(),
    vec![RespoEffect::new_insular(move |_, _dispatch, _el| {
      log_1(&format!("panel effect {:?}", cursor).into());
      Ok(())
    })],
    Box::new(
      div()
        .add_children([
          input()
            .class(ui_input())
            .insert_attr("placeholder", "some content...")
            .insert_attr("value", state.content.to_owned())
            .on_input(on_input)
            .to_owned(),
          space(Some(8), None),
          button().class(ui_button()).inner_text("add").on_click(on_submit).to_owned(),
          span().inner_text(format!("got panel state: {:?}", state)).to_owned(),
        ])
        .to_owned(),
    ),
  ))
}
