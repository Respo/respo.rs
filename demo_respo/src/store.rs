use std::{cell::RefCell, hash::Hash, rc::Rc};

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum IntentOp {
  #[default]
  Noop,
  IncTwice,
}

impl IntentOp {
  pub fn update(&self, store_to_action: Rc<RefCell<Store>>) -> Result<(), String> {
    use IntentOp::*;
    let mut store = store_to_action.borrow_mut();
    match self {
      Noop => Ok(()),
      IncTwice => {
        util::log!("inc twice");
        store.update(ActionOp::Increment)?;
        util::log!("inc twice {}", store.counted);
        store.update(ActionOp::Increment)?;
        util::log!("inc twice {}", store.counted);
        store.update(ActionOp::Increment)?;
        util::log!("inc twice {}", store.counted);
        Ok(())
      }
    }
  }
}

#[derive(Clone, Debug, Default)]
pub enum ActionOp {
  #[default]
  Noop,
  /// contains State and Value
  StatesChange(RespoUpdateState),
  Intent(IntentOp),
  Increment,
  Decrement,
  AddTask(String, String),
  RemoveTask(String),
  UpdateTask(String, String),
  ToggleTask(String),
}

impl RespoAction for ActionOp {
  type Intent = IntentOp;
  fn states_action(a: RespoUpdateState) -> Self {
    Self::StatesChange(a)
  }

  fn detect_intent(&self) -> Option<<ActionOp as RespoAction>::Intent> {
    match self {
      ActionOp::Intent(i) => Some(i.clone()),
      _ => None,
    }
  }

  fn build_intent_action(op: <ActionOp as RespoAction>::Intent) -> Self
  where
    Self: Sized,
  {
    Self::Intent(op)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&mut self) -> &mut RespoStatesTree {
    &mut self.states
  }

  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    use ActionOp::*;
    match op {
      Noop => {} // nothing to to
      StatesChange(a) => self.update_states(a),
      Intent(_i) => {
        unreachable!("intent should be handled in dispatch")
      }
      Increment => {
        self.counted += 1;
      }
      Decrement => {
        self.counted -= 1;
      }
      AddTask(id, content) => self.tasks.push(Task {
        id,
        content,
        time: 0.0,
        done: false,
      }),
      RemoveTask(id) => {
        self.tasks.retain(|task| task.id != id);
      }
      UpdateTask(id, content) => {
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
      ToggleTask(id) => {
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
