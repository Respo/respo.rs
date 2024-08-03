//! a demo for switching inner-text and children, which might cause a bug in respo

use std::fmt::Debug;

use respo::{button, css::respo_style, div, span, ui::ui_button, util, DispatchFn, RespoElement, RespoEvent};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use respo::states_tree::{RespoState, RespoStatesTree};

use super::store::ActionOp;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RespoState)]
struct InnerTextState {
  inner_text: bool,
}

pub fn comp_inner_text(states: &RespoStatesTree) -> Result<RespoElement<ActionOp>, String> {
  let cursor = states.path();

  let state = states.cast_branch::<InnerTextState>();

  let on_inc = {
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);
      if let RespoEvent::Click { original_event, .. } = e {
        original_event.prevent_default();
      }

      dispatch.run(ActionOp::Increment)?;
      dispatch.run_state(
        &cursor,
        InnerTextState {
          inner_text: !state.inner_text,
        },
      )?;
      Ok(())
    }
  };

  Ok(
    div().elements([
      div().elements([button()
        .class(ui_button())
        .inner_text("Switch inner text")
        .style(respo_style().margin(4))
        .on_click(on_inc)]),
      div().elements([if state.inner_text {
        div().inner_text("inner text")
      } else {
        div().elements([span().inner_text("child 1"), span().inner_text("child 2")])
      }]),
    ]),
  )
}
