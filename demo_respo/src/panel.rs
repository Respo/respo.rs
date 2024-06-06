use std::fmt::Debug;

use uuid::Uuid;

use crate::store::ActionOp;

use respo::{
  button, div, input, space, span,
  ui::{ui_button, ui_input},
  util, DispatchFn, RespoEvent, RespoNode, StatesTree,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PanelState {
  content: String,
}

pub fn comp_panel(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = states.data.cast_or_default::<PanelState>()?;

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
            .end(),
          space(Some(8), None),
          button().class(ui_button()).inner_text("add").on_click(on_submit).end(),
          span().inner_text(format!("got panel state: {:?}", state)).end(),
        ])
        .end(),
    )
    .stable_effect(move |_, _dispatch, _el| {
      respo::util::log!("panel effect {:?}", cursor);
      Ok(())
    })
    .end(),
  )
}
