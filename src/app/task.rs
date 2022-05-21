use std::{fmt::Debug, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
  button, input,
  respo::{declare_static_style, div, span, CssColor, RespoEffect, RespoEffectHandler, RespoNode, RespoStyle, StatesTree},
  ui::{ui_button, ui_center, ui_input, ui_row, ui_row_middle},
  util, CssSize, RespoEvent,
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
      RespoStyle::default()
        .margin(4.)
        .background_color(CssColor::Hsla(200., 90., 96., 1.)),
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
        .background_color(CssColor::Hsla(20., 90., 70., 1.)),
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
          .color(CssColor::Hsla(0., 90., 90., 1.)),
      ),
      ("$0:hover".to_owned(), RespoStyle::default().color(CssColor::Hsla(0., 90., 80., 1.))),
    ],
  )
}

pub fn comp_task(states: &StatesTree, task: &Task) -> Result<RespoNode<ActionOp>, String> {
  let task_id = task.id.to_owned();
  let task_id2 = task_id.clone();
  let task_id3 = task_id.clone();

  let cursor = states.path();
  let cursor2 = cursor.clone();
  let state = match &states.data {
    Some(v) => serde_json::from_value(v.to_owned()).expect("to task state"),
    None => TaskState::default(),
  };

  Ok(RespoNode::Component(
    "tasks".to_owned(),
    vec![RespoEffect {
      args: vec![serde_json::to_value(task).expect("to json")],
      handler: RespoEffectHandler(Rc::new(move |args, effect_type, el| -> Result<(), String> {
        let t: Task = serde_json::from_value(args[0].to_owned()).expect("from json");
        // TODO
        Ok(())
      })),
    }],
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
            .on_click(Rc::new(move |e, dispatch| -> Result<(), String> {
              dispatch.run(ActionOp::ToggleTask(task_id.clone()))?;
              Ok(())
            }))
            .to_owned(),
          div().add_attrs([("innerText", task.content.to_owned())]).to_owned(),
          span()
            .class_list(&[ui_center(), style_remove_button()])
            .insert_attr("innerText", "✕")
            .on_click(Rc::new(move |e, dispatch| -> Result<(), String> {
              util::log!("remove button {:?}", e);
              dispatch.run(ActionOp::RemoveTask(task_id2.to_owned()))?;
              Ok(())
            }))
            .to_owned(),
          div()
            .add_style(RespoStyle::default().margin4(0.0, 0.0, 0.0, 20.0).to_owned())
            .to_owned(),
          input()
            .class(ui_input())
            .insert_attr("value", state.draft.to_owned())
            .insert_attr("placeholder", "something to update...")
            .on_input(Rc::new(move |e, dispatch| -> Result<(), String> {
              if let RespoEvent::Input { value, .. } = e {
                dispatch.run(ActionOp::StatesChange(
                  cursor.to_owned(),
                  Some(serde_json::to_value(TaskState { draft: value }).expect("to json")),
                ))?;
              }
              Ok(())
            }))
            .to_owned(),
          button()
            .class(ui_button())
            .insert_attr("innerText", "Update")
            .on_click(Rc::new(move |e, dispatch| -> Result<(), String> {
              dispatch.run(ActionOp::UpdateTask(task_id3.to_owned(), state.draft.clone()))?;
              dispatch.run(ActionOp::StatesChange(cursor2.to_owned(), None))?;
              Ok(())
            }))
            .to_owned(),
        ])
        .to_owned(),
    ),
  ))
}
