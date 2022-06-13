use serde::{Deserialize, Serialize};

use respo::{util, MaybeState, RespoAction, RespoStore, StatesTree};

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
  StatesChange(Vec<String>, MaybeState),
  AddTask(String, String),
  RemoveTask(String),
  UpdateTask(String, String),
  ToggleTask(String),
}

impl RespoAction for ActionOp {
  fn wrap_states_action(cursor: &[String], a: MaybeState) -> Self {
    Self::StatesChange(cursor.to_vec(), a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&self) -> StatesTree {
    self.states.to_owned()
  }
  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      ActionOp::Increment => {
        self.counted += 1;
      }
      ActionOp::Decrement => {
        self.counted -= 1;
      }
      ActionOp::StatesChange(path, new_state) => {
        self.states.set_in_mut(&path, new_state);
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
}
