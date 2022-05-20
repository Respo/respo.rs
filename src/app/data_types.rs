use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::respo::StatesTree;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
  pub counted: i32,
  pub tasks: Vec<Task>,
  pub states: StatesTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub done: bool,
  pub content: String,
  pub time: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
  Increment,
  Decrement,
  StatesChange(Vec<String>, Option<Value>),
}
