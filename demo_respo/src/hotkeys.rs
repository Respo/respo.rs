use std::rc::Rc;

use respo::css::{respo_style, CssBorderStyle, CssColor, CssDisplay};
use respo::states_tree::{RespoState, RespoStatesTree};
use respo::{div, downcast_event, global_event_handler, request_rerender, span, DispatchFn, RespoComponent, RespoNode};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::window;

use crate::store::ActionOp;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, RespoState)]
struct ShortcutState {
  last_combo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotkeyEvent {
  pub key: String,
  pub ctrl: bool,
  pub meta: bool,
  pub alt: bool,
  pub shift: bool,
}

impl HotkeyEvent {
  pub fn new(key: impl Into<String>, ctrl: bool, meta: bool, alt: bool, shift: bool) -> Self {
    Self {
      key: key.into(),
      ctrl,
      meta,
      alt,
      shift,
    }
  }

  pub fn combo_label(&self) -> String {
    let mut parts: Vec<String> = Vec::new();
    if self.meta {
      parts.push("Cmd".to_owned());
    }
    if self.ctrl {
      parts.push("Ctrl".to_owned());
    }
    if self.alt {
      parts.push("Alt".to_owned());
    }
    if self.shift {
      parts.push("Shift".to_owned());
    }
    parts.push(self.key.to_uppercase());
    parts.join("+")
  }
}

pub fn comp_hotkey_demo(states: &RespoStatesTree) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = states.cast_branch::<ShortcutState>();
  let listener_cursor = cursor.to_owned();

  let listener = global_event_handler(move |event, ctx| {
    if let Some(evt) = downcast_event::<HotkeyEvent>(event) {
      let next_state = ShortcutState {
        last_combo: Some(evt.combo_label()),
      };
      ctx.dispatch.run_state(&listener_cursor, next_state)?;
      schedule_reset(ctx.dispatch.to_owned(), listener_cursor.clone())?;
    }
    Ok(())
  });

  let last_combo = state
    .last_combo
    .as_ref()
    .map(|s| format!("Last shortcut: {s}"))
    .unwrap_or_else(|| "Press Cmd/Ctrl + K to broadcast".to_owned());

  Ok(
    RespoComponent::named(
      "hotkey-demo",
      div()
        .style(
          respo_style()
            .padding(16)
            .border(Some((1.0, CssBorderStyle::Dashed, CssColor::Hsl(0, 0, 73))))
            .border_radius(8.0),
        )
        .elements([
          span().inner_text("Global shortcut demo"),
          span()
            .style(respo_style().display(CssDisplay::Block).margin4(8, 0, 0, 0).font_size(14.0))
            .inner_text(last_combo.as_str()),
        ]),
    )
    .push_listener(listener)
    .to_node(),
  )
}

fn schedule_reset(dispatch: DispatchFn<ActionOp>, cursor: Vec<Rc<str>>) -> Result<(), String> {
  let window = window().ok_or_else(|| String::from("window is not available"))?;
  let timeout = Closure::wrap(Box::new(move || {
    if let Err(err) = dispatch.run_state(&cursor, ShortcutState { last_combo: None }) {
      respo::util::error_log!("reset hotkey state failed: {err}");
    } else {
      request_rerender();
    }
  }) as Box<dyn FnMut()>);

  window
    .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.as_ref().unchecked_ref(), 1000)
    .map_err(|e| format!("schedule hotkey reset failed: {e:?}"))?;
  timeout.forget();
  Ok(())
}
