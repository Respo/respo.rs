use std::hash::Hash;

use respo::{states_tree::RespoUpdateState, util, RespoAction, RespoStore};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use respo::states_tree::{RespoState, RespoStatesTree};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Store {
  pub counted: i32,
  pub tasks: Vec<Task>,
  pub states: RespoStatesTree,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, RespoState)]
pub struct Task {
  pub id: String,
  pub done: bool,
  pub content: String,
  pub time: f32,
}

impl Eq for Task {}

impl Hash for Task {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.done.hash(state);
    self.content.hash(state);
  }
}

#[derive(Clone, Debug, Default)]
pub enum ActionOp {
  #[default]
  Noop,
  Increment,
  Decrement,
  /// contains State and Value
  StatesChange(RespoUpdateState),
  AddTask(String, String),
  RemoveTask(String),
  UpdateTask(String, String),
  ToggleTask(String),
}

impl RespoAction for ActionOp {
  fn states_action(a: RespoUpdateState) -> Self {
    Self::StatesChange(a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      ActionOp::Noop => {
        // nothing to to
      }
      ActionOp::Increment => {
        self.counted += 1;
      }
      ActionOp::Decrement => {
        self.counted -= 1;
      }
      ActionOp::StatesChange(RespoUpdateState(path, new_state, val)) => {
        self.states.set_in_mut(&path, new_state, val);
      }
      ActionOp::AddTask(id, content) => self.tasks.push(Task {
        id,
        content,
        time: 0.0,
        done: false,
      }),
      ActionOp::RemoveTask(id) => {
        self.tasks.retain(|task| task.id != id);
      }
      ActionOp::UpdateTask(id, content) => {
        let mut found = false;
        for task in &mut self.tasks {
          if task.id == id {
            task.content.clone_from(&content);
            found = true;
          }
        }
        if !found {
          return Err(format!("task {} not found", id));
        }
      }
      ActionOp::ToggleTask(id) => {
        let mut found = false;
        for task in &mut self.tasks {
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

  fn to_string(&self) -> String {
    serde_json::to_string(&self).expect("to json")
  }

  fn try_from_string(s: &str) -> Result<Self, String>
  where
    Self: Sized,
  {
    serde_json::from_str(s).map_err(|e| format!("parse store: {}", e))
  }
}
