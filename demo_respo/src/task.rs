use std::fmt::Debug;

use memoize::memoize;
use respo::{
  button, div, input, space, span, static_styles,
  ui::{ui_button, ui_center, ui_input, ui_row_middle},
  util, CssColor, CssSize, DispatchFn, RespoEvent, RespoNode, RespoStyle, StatesTree,
};

use super::store::*;

#[derive(Debug, Clone, Default, Hash)]
struct TaskState {
  draft: String,
}

#[memoize(Capacity: 40)]
pub fn comp_task(
  // _memo_caches: MemoCache<RespoNode<ActionOp>>,
  states: StatesTree,
  task: Task,
) -> Result<RespoNode<ActionOp>, String> {
  respo::util::log!("calling task function");

  let task_id = task.id.to_owned();

  let cursor = states.path();
  let cursor2 = cursor.clone();
  let state = states.data.cast_or_default::<TaskState>()?;

  let on_toggle = {
    let tid = task_id.clone();
    move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run(ActionOp::ToggleTask(tid.to_owned()))?;
      Ok(())
    }
  };

  let on_input = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    if let RespoEvent::Input { value, .. } = e {
      dispatch.run_state(&cursor, TaskState { draft: value })?;
    }
    Ok(())
  };

  let on_remove = {
    let tid = task_id.clone();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("remove button {:?}", e);
      dispatch.run(ActionOp::RemoveTask(tid.to_owned()))?;
      Ok(())
    }
  };

  let on_update = {
    let tid = task_id.clone();
    let s = state.clone();
    move |_e, dispatch: DispatchFn<_>| -> Result<(), String> {
      dispatch.run(ActionOp::UpdateTask(tid.to_owned(), s.draft.clone()))?;
      dispatch.run_empty_state(&cursor2)?;
      Ok(())
    }
  };

  Ok(
    RespoNode::new_component(
      "task",
      div()
        .class_list(&[ui_row_middle(), style_task_container()])
        .children([
          div()
            .class(style_done_button())
            .modify_style(|s| {
              if task.done {
                s.background_color(CssColor::Blue);
              }
            })
            .on_click(on_toggle)
            .to_owned(),
          div().inner_text(task.content.to_owned()).to_owned(),
          span()
            .class_list(&[ui_center(), style_remove_button()])
            .inner_text("✕")
            .on_click(on_remove)
            .to_owned(),
          div()
            .style(RespoStyle::default().margin4(0.0, 0.0, 0.0, 20.0).to_owned())
            .to_owned(),
          input()
            .class(ui_input())
            .attribute("value", state.draft.to_owned())
            .attribute("placeholder", "something to update...")
            .on_input(on_input)
            .to_owned(),
          space(Some(8), None),
          button().class(ui_button()).inner_text("Update").on_click(on_update).to_owned(),
        ])
        .to_owned(),
    )
    .effect(&[task], move |args, effect_type, _el| -> Result<(), String> {
      let t: Task = args[0].cast_into()?;
      util::log!("effect {:?} task: {:?}", effect_type, t);
      // TODO
      Ok(())
    })
    .share_with_ref(),
  )
}

static_styles!(
  style_task_container,
  ("&", RespoStyle::default().margin(4.).background_color(CssColor::Hsl(200, 90, 96)),)
);

static_styles!(
  style_done_button,
  (
    "&",
    RespoStyle::default()
      .width(CssSize::Px(24.0))
      .height(CssSize::Px(24.0))
      .margin(4.)
      .cursor("pointer".to_owned())
      .background_color(CssColor::Hsl(20, 90, 70)),
  )
);

static_styles!(
  style_remove_button,
  (
    "&",
    RespoStyle::default()
      .width(CssSize::Px(16.0))
      .height(CssSize::Px(16.0))
      .margin(4.)
      .cursor("pointer".to_owned())
      .margin4(0.0, 0.0, 0.0, 16.0)
      .color(CssColor::Hsl(0, 90, 90)),
  ),
  ("$0:hover", RespoStyle::default().color(CssColor::Hsl(0, 90, 80))),
);
