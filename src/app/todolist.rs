use std::fmt::Debug;

use crate::respo::{div, span, RespoNode, StatesTree};

use super::{data_types::Task, task::comp_task};

pub fn comp_todolist<T>(states: &StatesTree, tasks: &Vec<Task>) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  Ok(
    div()
      .add_children([
        span().add_attrs([("innerText", format!("TODO {:?}", tasks))]).to_owned(),
        comp_task(
          &states.pick("task"),
          &Task {
            done: false,
            content: String::from("task 1"),
            time: 0.0,
          },
        )?,
      ])
      .to_owned(),
  )
}
