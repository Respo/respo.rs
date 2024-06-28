use std::fmt::Debug;

use respo::{
  br, button,
  css::{CssColor, RespoStyle},
  div, span,
  states_tree::RespoStatesTreeCasted,
  ui::ui_button,
  util, DispatchFn, RespoElement, RespoEvent,
};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use respo::states_tree::RespoState;

use crate::IntentOp;

use super::store::ActionOp;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RespoState)]
pub(crate) struct CounterState {
  counted: i32,
}

pub fn comp_counter(states: RespoStatesTreeCasted<CounterState>, global_counted: i32) -> Result<RespoElement<ActionOp>, String> {
  let cursor = states.path();

  let state = states.data;
  let counted = state.counted;

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
        CounterState {
          counted: state.counted + 2,
        },
      )?;
      Ok(())
    }
  };

  let on_dec = {
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);
      dispatch.run(ActionOp::Decrement)?;
      dispatch.run_state(
        &cursor,
        CounterState {
          counted: state.counted - 1,
        },
      )?;
      Ok(())
    }
  };

  let on_inc_twice = {
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);
      dispatch.run(ActionOp::Intent(IntentOp::IncTwice))?;
      dispatch.run_state(
        &cursor,
        CounterState {
          counted: state.counted + 2,
        },
      )?;
      Ok(())
    }
  };

  Ok(
    div().elements([
      div().elements([
        button()
          .class(ui_button())
          .inner_text("demo inc")
          .style(RespoStyle::default().margin(4.))
          .on_click(on_inc),
        button()
          .class(ui_button())
          .inner_text("demo dec")
          .style(RespoStyle::default().margin(4.))
          .on_click(on_dec),
        button()
          .class(ui_button())
          .inner_text("demo inc twice")
          .style(RespoStyle::default().margin(4.))
          .on_click(on_inc_twice),
      ]),
      div().elements([span().inner_text(format!("value is: {}", counted)).style(
        RespoStyle::default()
          .color(CssColor::Hsluv(270, 100, 40))
          .font_family("Menlo".to_owned())
          .font_size(10. + counted as f32),
      )]),
      div().elements([
        span().inner_text(format!("local state: {}", counted)),
        br(),
        span().inner_text(format!("global state: {}", global_counted)),
      ]),
    ]),
  )
}
