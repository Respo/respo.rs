use std::{fmt::Debug, rc::Rc};

use crate::{
  respo::{declare_static_style, div, span, CssColor, RespoEffect, RespoEffectHandler, RespoNode, RespoStyle, StatesTree},
  ui::{ui_center, ui_row, ui_row_middle},
  util, CssSize,
};

use super::data_types::*;

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
        ])
        .to_owned(),
    ),
  ))
}
