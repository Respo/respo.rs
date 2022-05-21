use std::{fmt::Debug, rc::Rc};

use crate::respo::{declare_static_style, div, span, CssColor, RespoEffect, RespoEffectHandler, RespoNode, RespoStyle, StatesTree};

use super::data_types::*;

pub fn style_task_container() -> String {
  declare_static_style(
    "task-comp",
    &[(
      "$0".to_owned(),
      RespoStyle::default()
        .margin(4.)
        .background_color(CssColor::Hsla(200., 90., 90., 1.)),
    )],
  )
}

pub fn comp_task<T>(states: &StatesTree, task: &Task) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
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
        .add_attrs([("class", style_task_container())])
        .add_children([span().add_attrs([("innerText", format!("TODO {:?}", task))]).to_owned()])
        .to_owned(),
    ),
  ))
}
