use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{respo::StatesTree, util};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
  pub counted: i32,
  pub tasks: Vec<Task>,
  pub states: StatesTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub done: bool,
  pub content: String,
  pub time: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
  Increment,
  Decrement,
  StatesChange(Vec<String>, Option<Value>),
  AddTask(String, String),
  RemoveTask(String),
  UpdateTask(String, String),
  ToggleTask(String),
}

pub fn apply_action(store: &mut Store, op: ActionOp) -> Result<(), String> {
  match op {
    ActionOp::Increment => {
      store.counted += 1;
    }
    ActionOp::Decrement => {
      store.counted -= 1;
    }
    ActionOp::StatesChange(path, new_state) => {
      store.states = store.states.set_in(&path, new_state);
    }
    ActionOp::AddTask(id, content) => store.tasks.push(Task {
      id,
      content,
      time: 0.0,
      done: false,
    }),
    ActionOp::RemoveTask(id) => {
      store.tasks.retain(|task| task.id != id);
    }
    ActionOp::UpdateTask(id, content) => {
      let mut found = false;
      for task in &mut store.tasks {
        if task.id == id {
          task.content = content.to_owned();
          found = true;
        }
      }
      if !found {
        return Err(format!("task {} not found", id));
      }
    }
    ActionOp::ToggleTask(id) => {
      let mut found = false;
      for task in &mut store.tasks {
        if task.id == id {
          util::log!("change task {:?}", task);
          task.done = !task.done;
          found = true;
        }
      }
      if !found {
        return Err(format!("task {} not found", id));
      }
    }
  }
  Ok(())
}
