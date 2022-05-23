use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
  button, input,
  respo::{declare_static_style, div, span, CssColor, RespoEffect, RespoNode, RespoStyle, StatesTree},
  space,
  ui::{ui_button, ui_center, ui_input, ui_row_middle},
  util::{self, cast_from_json, cast_into_json},
  CssSize, DispatchFn, RespoEvent,
};

use super::data_types::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TaskState {
  draft: String,
}

pub fn style_task_container() -> String {
  declare_static_style(
    "task-comp",
    &[(
      "$0".to_owned(),
      RespoStyle::default().margin(4.).background_color(CssColor::Hsl(200, 90, 96)),
    )],
  )
}

pub fn style_done_button() -> String {
  declare_static_style(
    "task-done-button",
    &[(
      "$0".to_owned(),
      RespoStyle::default()
        .width(CssSize::Px(24.0))
        .height(CssSize::Px(24.0))
        .margin(4.)
        .cursor("pointer".to_owned())
        .background_color(CssColor::Hsl(20, 90, 70)),
    )],
  )
}

pub fn style_remove_button() -> String {
  declare_static_style(
    "task-remove-button",
    &[
      (
        "$0".to_owned(),
        RespoStyle::default()
          .width(CssSize::Px(16.0))
          .height(CssSize::Px(16.0))
          .margin(4.)
          .cursor("pointer".to_owned())
          .margin4(0.0, 0.0, 0.0, 16.0)
          .color(CssColor::Hsl(0, 90, 90)),
      ),
      ("$0:hover".to_owned(), RespoStyle::default().color(CssColor::Hsl(0, 90, 80))),
    ],
  )
}

pub fn comp_task(states: &StatesTree, task: &Task) -> Result<RespoNode<ActionOp>, String> {
  let task_id = task.id.to_owned();
  let task_id2 = task_id.clone();
  let task_id3 = task_id.clone();

  let cursor = states.path();
  let cursor2 = cursor.clone();
  let state = states.data.as_ref().map(cast_from_json::<TaskState>).unwrap_or_default();
  let state2 = state.clone();

  let on_toggle = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::ToggleTask(task_id.to_owned()))?;
    Ok(())
  };

  let on_input = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run(ActionOp::StatesChange(
        cursor.to_owned(),
        Some(cast_into_json(TaskState { draft: value })),
      ))?;
    }
    Ok(())
  };

  let on_remove = move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
    dispatch.run(ActionOp::UpdateTask(task_id3.to_owned(), state2.draft.clone()))?;
    dispatch.run(ActionOp::StatesChange(cursor2.to_owned(), None))?;
    Ok(())
  };

  Ok(RespoNode::Component(
    "tasks".to_owned(),
    vec![RespoEffect::new(
      vec![cast_into_json(task)],
      move |args, effect_type, _el| -> Result<(), String> {
        let t: Task = cast_from_json(&args[0]);
        util::log!("effect {:?} task: {:?}", effect_type, t);
        // TODO
        Ok(())
      },
    )],
    Box::new(
      div()
        .class_list(&[ui_row_middle(), style_task_container()])
        .add_children([
          div()
            .class(style_done_button())
            .add_style(if task.done {
              RespoStyle::default().background_color(CssColor::Blue).to_owned()
            } else {
              RespoStyle::default()
            })
            .on_click(on_toggle)
            .to_owned(),
          div().inner_text(task.content.to_owned()).to_owned(),
          span()
            .class_list(&[ui_center(), style_remove_button()])
            .inner_text("✕")
            .on_click(move |e, dispatch| -> Result<(), String> {
              util::log!("remove button {:?}", e);
              dispatch.run(ActionOp::RemoveTask(task_id2.to_owned()))?;
              Ok(())
            })
            .to_owned(),
          div()
            .add_style(RespoStyle::default().margin4(0.0, 0.0, 0.0, 20.0).to_owned())
            .to_owned(),
          input()
            .class(ui_input())
            .insert_attr("value", state.draft)
            .insert_attr("placeholder", "something to update...")
            .on_input(on_input)
            .to_owned(),
          space(Some(8), None),
          button().class(ui_button()).inner_text("Update").on_click(on_remove).to_owned(),
        ])
        .to_owned(),
    ),
  ))
}
