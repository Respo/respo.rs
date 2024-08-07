use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoElement, RespoIndexKey, RespoNode};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use respo::states_tree::{RespoState, RespoStatesTree};

use super::{
  store::{ActionOp, Task},
  task::comp_task,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RespoState)]
struct TodolistState {
  hide_done: bool,
}

pub fn comp_todolist(states: &RespoStatesTree, tasks: &[Task]) -> Result<RespoElement<ActionOp>, String> {
  let cursor = states.path();
  let state = states.cast_branch::<TodolistState>();

  let mut children: Vec<(RespoIndexKey, RespoNode<_>)> = vec![];
  for task in tasks {
    if state.hide_done && task.done {
      continue;
    }

    children.push(((&task.id).into(), comp_task(states.pick(&task.id), task.to_owned())?));
  }

  // util::log!("{:?}", &tasks);

  let on_hide = {
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      dispatch.run_state(
        &cursor,
        TodolistState {
          hide_done: !state.hide_done,
        },
      )?;

      Ok(())
    }
  };

  Ok(div().elements([
    div().elements([
      span().inner_text(format!("tasks size: {} ... {}", tasks.len(), state.hide_done)),
      button().class(ui_button()).inner_text("hide done").on_click(on_hide),
    ]),
    div().children_indexed(children),
  ]))
}
