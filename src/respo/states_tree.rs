use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatesTree {
  /// local data
  pub data: Option<Value>,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<String>,
  /// holding children states
  pub branches: HashMap<String, Box<StatesTree>>,
}

impl StatesTree {
  /// get cursor
  pub fn path(&self) -> Vec<String> {
    self.cursor.clone()
  }

  /// pick a child branch as new cursor
  pub fn pick(&self, name: &str) -> StatesTree {
    let mut next_cursor = self.cursor.clone();
    next_cursor.push(name.to_owned());

    if self.branches.contains_key(name) {
      let prev = &self.branches[name];
      Self {
        data: prev.data.clone(),
        cursor: next_cursor,
        branches: prev.branches.clone(),
      }
    } else {
      Self {
        data: None,
        cursor: next_cursor,
        branches: HashMap::new(),
      }
    }
  }

  /// in-place mutation of state tree
  pub fn set_in_mut(&mut self, path: &[String], new_state: Option<Value>) {
    if path.is_empty() {
      (*self).data = new_state;
    } else {
      let (p_head, p_rest) = path.split_at(1);
      let p0 = p_head[0].to_owned();
      if let Some(branch) = self.branches.get_mut(&p0) {
        branch.set_in_mut(p_rest, new_state);
      } else {
        let mut branch = self.pick(&p0);
        branch.set_in_mut(p_rest, new_state);
        self.branches.insert(p0, Box::new(branch));
      }
    }
  }
}
