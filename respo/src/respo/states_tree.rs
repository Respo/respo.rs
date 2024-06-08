use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::{log, DynEq};

// use wasm_bindgen::JsValue;

/// Respo maintains states in a tree structure, where the keys are strings,
/// each child component "picks" a key to attach its own state to the tree,
/// and it dispatches events to global store to update the state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatesTree {
  /// local data
  #[serde(skip)]
  pub data: Option<RespoStateBranch>,
  pub backup: Option<Value>,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<Rc<str>>,
  // pub data_type_name: Option<TypeId>,
  // pub data_revision: usize,
  /// holding children states
  pub branches: BTreeMap<Rc<str>, Box<StatesTree>>,
}

impl Hash for StatesTree {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.cursor.hash(state);
    self.data.hash(state);
    // backup is not real data
    self.branches.hash(state);
  }
}

impl PartialEq for StatesTree {
  fn eq(&self, other: &Self) -> bool {
    // backup is only for backup
    // this trick might cause inconsistency in some cases after reloaded
    self.cursor == other.cursor && self.data == other.data && self.branches == other.branches
  }
}
impl Eq for StatesTree {}

impl StatesTree {
  /// get cursor
  pub fn path(&self) -> Vec<Rc<str>> {
    self.cursor.to_owned()
  }

  /// get shared data from state tree. fallback to backup and then default
  pub fn cast_branch<T>(&self) -> Result<Rc<T>, String>
  where
    T: Clone + Default + RespoState + 'static,
  {
    if let Some(v) = &self.data {
      if let Some(v) = v.0.as_ref().as_any().downcast_ref::<T>() {
        return Ok(Rc::new(v.to_owned()));
      } else {
        log!("failed to cast state to {}", std::any::type_name::<T>());
      }
    }

    match &self.backup {
      Some(v) => {
        let mut t = T::default();
        match t.restore_from(v) {
          Ok(_) => Ok(Rc::new(t)),
          Err(e) => Err(e),
        }
      }
      None => Ok(Rc::new(T::default())),
    }
  }

  /// pick a child branch as new cursor
  pub fn pick(&self, name: &str) -> StatesTree {
    let mut next_cursor = self.cursor.to_owned();
    next_cursor.push(Rc::from(name));

    if self.branches.contains_key(name) {
      let prev = &self.branches[name];
      Self {
        data: prev.data.to_owned(),
        backup: prev.backup.to_owned(),
        // data_revision: prev.data_revision,
        // data_type_name: prev.data_type_name.to_owned(),
        cursor: next_cursor,
        branches: prev.branches.to_owned(),
      }
    } else {
      Self {
        data: None,
        backup: None,
        cursor: next_cursor,
        // data_type_name: None,
        // data_revision: 0,
        branches: BTreeMap::new(),
      }
    }
  }

  /// in-place mutation of state tree
  pub fn set_in_mut(&mut self, path: &[Rc<str>], new_state: Option<RespoStateBranch>, val: Option<Value>) {
    if path.is_empty() {
      new_state.clone_into(&mut self.data);
      val.clone_into(&mut self.backup);
      // self.data_type_name = new_state.0.as_ref().map(|v| v.type_id().to_owned());
      // self.data_revision += 1;
    } else {
      let (p_head, p_rest) = path.split_at(1);
      let p0 = &p_head[0];
      if let Some(branch) = self.branches.get_mut(p0) {
        branch.set_in_mut(p_rest, new_state, val);
      } else {
        let mut branch = self.pick(p0);
        branch.set_in_mut(p_rest, new_state, val);
        self.branches.insert(p0.to_owned(), Box::new(branch));
      }
    }
  }
}

#[derive(Debug, Clone)]
/// local state in component could be `None` according to the tree structure
pub struct RespoStateBranch(pub Rc<dyn DynEq>);

impl PartialEq for RespoStateBranch {
  fn eq(&self, other: &Self) -> bool {
    self.0.as_ref().do_eq(other.0.as_ref())
  }
}
impl Eq for RespoStateBranch {}

impl Hash for RespoStateBranch {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    state.write_usize(Rc::as_ptr(&self.0) as *const () as usize);
  }
}

impl RespoStateBranch {
  pub fn new(state: Rc<dyn DynEq>) -> Self {
    Self(state)
  }
}

/// component level state that could be backuped
pub trait RespoState {
  fn backup(&self) -> Option<Value> {
    None
  }
  fn restore_from(&mut self, _s: &Value) -> Result<(), String> {
    Ok(())
  }
}

impl RespoState for bool {
  fn backup(&self) -> Option<Value> {
    Some(Value::Bool(*self))
  }

  fn restore_from(&mut self, s: &Value) -> Result<(), String> {
    *self = s.as_bool().unwrap();
    Ok(())
  }
}

impl RespoState for () {
  fn backup(&self) -> Option<Value> {
    None
  }

  fn restore_from(&mut self, _s: &Value) -> Result<(), String> {
    Ok(())
  }
}
