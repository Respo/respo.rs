use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
  button, memo1_call_by,
  respo::{div, span, RespoNode, StatesTree},
  ui::ui_button,
  util::{self, cast_from_json, cast_into_json},
  MemoCache, RespoIndexKey,
};

use super::{
  data_types::{ActionOp, Task},
  task::comp_task,
};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TodolistState {
  hide_done: bool,
}

pub fn comp_todolist(
  memo_caches: Rc<RefCell<MemoCache<RespoNode<ActionOp>>>>,
  states: &StatesTree,
  tasks: &[Task],
) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = states.data.as_ref().map(cast_from_json::<TodolistState>).unwrap_or_default();

  let mut children: Vec<(RespoIndexKey, RespoNode<_>)> = vec![];
  for task in tasks {
    if state.hide_done && task.done {
      continue;
    }
    // children.push((
    //   task.id.to_owned().into(),
    //   comp_task(memo_caches.to_owned(), &states.pick(&task.id), task)?,
    // ));

    let m = memo_caches.to_owned();

    // children.push((
    //   task.id.to_owned().into(),
    //   internal_memof1_call_by(
    //     memo_caches.to_owned(),
    //     comp_task as usize,
    //     task.id.to_owned(),
    //     vec![cast_into_json(states.pick(&task.id)), cast_into_json(task)],
    //     move || comp_task(m.to_owned(), &states.pick(&task.id), task),
    //   )?,
    // ));

    children.push((
      task.id.to_owned().into(),
      // comp_task(memo_caches.to_owned(), &states.pick(&task.id), task)?,
      memo1_call_by!(comp_task, m.to_owned(), task.id.to_owned(), &states.pick(&task.id), task)?,
    ));
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

                dispatch.run_state(
                  &cursor,
                  cast_into_json(TodolistState {
                    hide_done: !state.hide_done,
                  }),
                )?;
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
