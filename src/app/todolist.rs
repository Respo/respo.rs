use crate::respo::{div, span, RespoNode, StatesTree};

use super::{
  data_types::{ActionOp, Task},
  task::comp_task,
};

pub fn comp_todolist(states: &StatesTree, tasks: &Vec<Task>) -> Result<RespoNode<ActionOp>, String> {
  let mut children = vec![];
  for task in tasks {
    children.push((task.id.to_owned().into(), comp_task(states, task)?));
  }

  // util::log!("{:?}", &tasks);

  Ok(
    div()
      .add_children([span().add_attrs([("innerText", format!("tasks size: {}", tasks.len()))]).to_owned()])
      .add_children_indexed(children)
      .to_owned(),
  )
}
