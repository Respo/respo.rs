use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use memoize::memoize;
use respo::{
  button,
  css::{
    respo_style, ConvertRespoCssSize,
    CssColor::{self, Hsl},
  },
  div, input, space, span, static_styles,
  ui::{ui_button, ui_center, ui_input, ui_row_middle},
  util, DispatchFn, RespoComponent, RespoEffect, RespoEvent, RespoNode,
};

use respo::states_tree::{RespoState, RespoStatesTree};

use super::store::*;

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq, Serialize, Deserialize, RespoState)]
struct TaskState {
  draft: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TaskUpdateEffect {
  task: Task,
}

impl RespoEffect for TaskUpdateEffect {
  fn updated(&self, _el: &web_sys::Node) -> Result<(), String> {
    util::log!("task update effect");
    Ok(())
  }
}

#[memoize(Capacity: 40)]
pub fn comp_task(
  // _memo_caches: MemoCache<RespoNode<ActionOp>>,
  states: RespoStatesTree,
  task: Task,
) -> Result<RespoNode<ActionOp>, String> {
  respo::util::log!("calling task function");

  let task_id = &task.id;

  let cursor = states.path();
  let state = states.cast_branch::<TaskState>();

  let on_toggle = {
    let tid = task_id.to_owned();
    move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run(ActionOp::ToggleTask(tid.to_owned()))?;
      Ok(())
    }
  };

  let on_input = {
    let cursor = cursor.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      if let RespoEvent::Input { value, .. } = e {
        dispatch.run_state(&cursor, TaskState { draft: value })?;
      }
      Ok(())
    }
  };

  let on_remove = {
    let tid = task_id.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("remove button {:?}", e);
      dispatch.run(ActionOp::RemoveTask(tid.to_owned()))?;
      Ok(())
    }
  };

  let on_update = {
    let tid = task_id.to_owned();
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run(ActionOp::UpdateTask(tid.to_owned(), state.draft.to_owned()))?;
      dispatch.run_empty_state(&cursor)?;
      Ok(())
    }
  };

  Ok(
    RespoComponent::named(
      "task",
      div().class_list(&[ui_row_middle(), style_task_container()]).elements([
        div()
          .class(style_done_button())
          .modify_style(|s| {
            if task.done {
              *s = s.to_owned().background_color(CssColor::Blue);
            }
          })
          .on_click(on_toggle),
        div().inner_text(&task.content),
        span()
          .class_list(&[ui_center(), style_remove_button()])
          .inner_text("✕")
          .on_click(on_remove),
        div().style(respo_style().margin4(0, 0, 0, 20)),
        input()
          .attrs(&[("value", state.draft.as_str()), ("placeholder", "something to update...")])
          .class(ui_input())
          .on_input(on_input),
        space(Some(8), None),
        button().class(ui_button()).inner_text("Update").on_click(on_update),
      ]),
    )
    .effect(TaskUpdateEffect { task: task.to_owned() })
    .to_node()
    .rc(),
  )
}

static_styles!(
  style_task_container,
  ("&", respo_style().margin(4).background_color(Hsl(200, 90, 96)))
);

static_styles!(
  style_done_button,
  (
    "&",
    respo_style()
      .width(24.px())
      .height(24.px())
      .margin(4)
      .cursor("pointer")
      .background_color(Hsl(20, 90, 70)),
  )
);

static_styles!(
  style_remove_button,
  (
    "&",
    respo_style()
      .width(16.px())
      .height(16.px())
      .cursor("pointer")
      .margin4(0, 0, 0, 16)
      .color(Hsl(0, 90, 90)),
  ),
  ("$0:hover", respo_style().color(Hsl(0, 90, 80))),
);
