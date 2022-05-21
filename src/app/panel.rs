use std::{fmt::Debug, rc::Rc};

use web_sys::console::log_1;

use crate::respo::{div, span, util, RespoEffect, RespoEffectHandler, RespoNode, StatesTree};

pub fn comp_panel<T>(states: &StatesTree) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  Ok(RespoNode::Component(
    "panel".to_owned(),
    vec![RespoEffect {
      args: vec![],
      handler: RespoEffectHandler(Rc::new(move |args, action_type, el| -> Result<(), String> {
        util::log!("panel mounted");
        Ok(())
      })),
    }],
    Box::new(
      div()
        .add_children([span().add_attrs([("innerText", String::from("TODO panel"))]).to_owned()])
        .to_owned(),
    ),
  ))
}
