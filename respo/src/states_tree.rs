//! Respo does not provide local states in components, but a global states tree.
//! `RespoStatesTree` tree has concept of "cursor", which is a path to the current state in the tree.
//! use `branch.pick(name)` to get a child branch, and `branch.set_in_mut(change)` to update the tree.

mod dyn_eq;
mod state;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::warn_log;
pub(crate) use dyn_eq::DynEq;

pub use state::RespoState;

// use wasm_bindgen::JsValue;

/// Respo maintains states in a tree structure, where the keys are strings,
/// each child component "picks" a key to attach its own state to the tree,
/// and it dispatches events to global store to update the state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RespoStatesTree {
  /// local data
  #[serde(skip)]
  pub data: Option<RespoStateBranch>,
  pub backup: Option<Value>,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<Rc<str>>,
  // pub data_type_name: Option<TypeId>,
  // pub data_revision: usize,
  /// holding children states
  pub branches: BTreeMap<Rc<str>, Box<RespoStatesTree>>,
}

impl Hash for RespoStatesTree {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.cursor.hash(state);
    self.data.hash(state);
    // backup is not real data
    self.branches.hash(state);
  }
}

impl PartialEq for RespoStatesTree {
  fn eq(&self, other: &Self) -> bool {
    // backup is only for backup
    // this trick might cause inconsistency in some cases after reloaded
    self.cursor == other.cursor && self.data == other.data && self.branches == other.branches
  }
}
impl Eq for RespoStatesTree {}

impl RespoStatesTree {
  /// get cursor
  pub fn path(&self) -> Vec<Rc<str>> {
    self.cursor.to_owned()
  }

  /// get shared data from state tree. fallback to backup and then default
  pub fn cast_branch<T>(&self) -> Rc<T>
  where
    T: Clone + Default + RespoState + 'static,
  {
    if let Some(v) = &self.data {
      if let Some(v) = v.0.as_ref().as_any().downcast_ref::<T>() {
        return Rc::new(v.to_owned());
      } else {
        warn_log!("failed to cast state to {} , at {:?}", std::any::type_name::<T>(), self.cursor);
      }
    }

    match &self.backup {
      Some(v) => {
        let mut t = T::default();
        if let Err(e) = t.restore_from(v) {
          warn_log!("failed to restore from backup: {} , at {:?}", e, self.cursor);
        }
        Rc::new(t)
      }
      None => Rc::new(T::default()),
    }
  }

  /// pick a child branch as new cursor
  pub fn pick(&self, name: &str) -> RespoStatesTree {
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
  pub(crate) fn set_in_mut(&mut self, change: RespoUpdateState) {
    if change.cursor.is_empty() {
      change.data.clone_into(&mut self.data);
      change.backup.clone_into(&mut self.backup);
      // self.data_type_name = new_state.0.as_ref().map(|v| v.type_id().to_owned());
      // self.data_revision += 1;
    } else {
      let (p_head, p_rest) = change.cursor.split_at(1);
      let p0 = &p_head[0];
      if let Some(branch) = self.branches.get_mut(p0) {
        branch.set_in_mut(RespoUpdateState {
          cursor: p_rest.to_vec(),
          ..change
        });
      } else {
        let mut branch = self.pick(p0);
        branch.set_in_mut(RespoUpdateState {
          cursor: p_rest.to_vec(),
          ..change
        });
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

/// framework defined action for updating states branch
#[derive(Clone, Debug)]
pub struct RespoUpdateState {
  /// path to the state
  pub cursor: Vec<Rc<str>>,
  /// dyn eq data
  pub data: Option<RespoStateBranch>,
  /// backup data for restoring
  pub backup: Option<Value>,
}
