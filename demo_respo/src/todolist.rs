use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoIndexKey, RespoNode, StatesTree};

use super::{
  store::{ActionOp, Task},
  task::comp_task,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TodolistState {
  hide_done: bool,
}

pub fn comp_todolist(states: &StatesTree, tasks: &[Task]) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();
  let state = states.data.cast_or_default::<TodolistState>()?;

  let mut children: Vec<(RespoIndexKey, RespoNode<_>)> = vec![];
  for task in tasks {
    if state.hide_done && task.done {
      continue;
    }
    // children.push((
    //   task.id.to_owned().into(),
    //   comp_task(memo_caches.to_owned(), &states.pick(&task.id), task)?,
    // ));

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
      RespoIndexKey::from(&task.id),
      // comp_task(memo_caches.to_owned(), &states.pick(&task.id), task)?,
      comp_task(states.pick(&task.id), task.to_owned())?,
    ));
  }

  // util::log!("{:?}", &tasks);

  let on_hide = {
    let s = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      dispatch.run_state(&cursor, TodolistState { hide_done: !s.hide_done })?;

      Ok(())
    }
  };

  Ok(
    div()
      .children([
        div()
          .children([
            span()
              .inner_text(format!("tasks size: {} ... {}", tasks.len(), state.hide_done.to_owned()))
              .to_owned(),
            button().class(ui_button()).inner_text("hide done").on_click(on_hide).to_owned(),
          ])
          .to_owned(),
        div().children_indexed(children).to_owned(),
      ])
      .to_owned(),
  )
}
