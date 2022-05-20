use std::collections::HashMap;
use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatesTree {
  pub data: Option<Value>,
  cursor: Vec<String>,
  branches: HashMap<String, Box<StatesTree>>,
}

impl StatesTree {
  pub fn path(&self) -> Vec<String> {
    self.cursor.clone()
  }

  // pick a child branch as new cursor
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

  /// returns a new tree
  pub fn set_in(&self, path: &[String], new_state: Option<Value>) -> Self {
    if path.is_empty() {
      Self {
        data: new_state,
        cursor: self.cursor.clone(),
        branches: self.branches.clone(),
      }
    } else {
      let (p_head, p_rest) = path.split_at(1);
      let p0 = p_head[0].to_owned();
      if self.branches.contains_key(&p0) {
        let branch = self.branches[&p0].clone();
        let next_branch = branch.set_in(p_rest, new_state);

        let mut next = self.clone();
        next.branches.insert(p0, Box::new(next_branch));
        next
      } else {
        let mut next = self.clone();
        next.branches.insert(p0, Box::new(StatesTree::pick(self, &path[0])));
        next
      }
    }
  }
}
