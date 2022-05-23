use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
  button,
  respo::{div, span, RespoNode, StatesTree},
  ui::ui_button,
  util,
};

use super::{
  data_types::{ActionOp, Task},
  task::comp_task,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TodolistState {
  hide_done: bool,
}

pub fn comp_todolist(states: &StatesTree, tasks: &[Task]) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = match &states.data {
    Some(v) => serde_json::from_value(v.to_owned()).map_err(|e| format!("to todolist state: {}", e))?,
    None => TodolistState::default(),
  };

  let mut children = vec![];
  for task in tasks {
    if state.hide_done && task.done {
      continue;
    }
    children.push((task.id.to_owned().into(), comp_task(&states.pick(&task.id), task)?));
  }

  // util::log!("{:?}", &tasks);

  Ok(
    div()
      .add_children([
        div()
          .add_children([
            span()
              .inner_text(format!("tasks size: {} ... {}", tasks.len(), state.hide_done))
              .to_owned(),
            button()
              .class(ui_button())
              .inner_text("hide done")
              .on_click(move |e, dispatch| -> Result<(), String> {
                util::log!("click {:?}", e);

                dispatch.run(ActionOp::StatesChange(
                  cursor.to_owned(),
                  Some(
                    serde_json::to_value(TodolistState {
                      hide_done: !state.hide_done,
                    })
                    .expect("to json"),
                  ),
                ))?;
                Ok(())
              })
              .to_owned(),
          ])
          .to_owned(),
        div().add_children_indexed(children).to_owned(),
      ])
      .to_owned(),
  )
}
