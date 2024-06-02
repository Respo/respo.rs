use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::console::log_1;

use crate::store::ActionOp;

use respo::{
  button, div, input, space, span,
  ui::{ui_button, ui_input},
  util, DispatchFn, RespoEvent, RespoNode, StatesTree,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PanelState {
  content: String,
}

pub fn comp_panel(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let cursor2 = cursor.clone();
  let cursor3 = cursor.clone();
  let state = states.data.cast_or_default::<PanelState>()?;
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

  Ok(
    RespoNode::new_component(
      "panel",
      div()
        .children([
          input()
            .class(ui_input())
            .attribute("placeholder", "some content...")
            .attribute("value", state.content.to_owned())
            .on_input(on_input)
            .to_owned(),
          space(Some(8), None),
          button().class(ui_button()).inner_text("add").on_click(on_submit).to_owned(),
          span().inner_text(format!("got panel state: {:?}", state)).to_owned(),
        ])
        .to_owned(),
    )
    .stable_effect(move |_, _dispatch, _el| {
      log_1(&format!("panel effect {:?}", cursor).into());
      Ok(())
    })
    .to_owned(),
  )
}
