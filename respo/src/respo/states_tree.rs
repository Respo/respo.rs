use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use crate::DynEq;

// use wasm_bindgen::JsValue;
// use web_sys::console::log_1;

/// Respo maintains states in a tree structure, where the keys are strings,
/// each child component "picks" a key to attach its own state to the tree,
/// and it dispatches events to global store to update the state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct StatesTree {
  /// local data
  pub data: MaybeState,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<String>,
  // pub data_type_name: Option<TypeId>,
  // pub data_revision: usize,
  /// holding children states
  pub branches: BTreeMap<String, Box<StatesTree>>,
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
        // data_revision: prev.data_revision,
        // data_type_name: prev.data_type_name.to_owned(),
        cursor: next_cursor,
        branches: prev.branches.clone(),
      }
    } else {
      Self {
        data: MaybeState::new(None),
        cursor: next_cursor,
        // data_type_name: None,
        // data_revision: 0,
        branches: BTreeMap::new(),
      }
    }
  }

  /// in-place mutation of state tree
  pub fn set_in_mut(&mut self, path: &[String], new_state: MaybeState) {
    if path.is_empty() {
      new_state.clone_into(&mut self.data);
      // self.data_type_name = new_state.0.as_ref().map(|v| v.type_id().to_owned());
      // self.data_revision += 1;
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

#[derive(Debug, Clone, Default)]
/// local state in component could be `None` according to the tree structure
pub struct MaybeState(Option<Rc<dyn DynEq>>);

impl PartialEq for MaybeState {
  fn eq(&self, other: &Self) -> bool {
    match (&self.0, &other.0) {
      (None, None) => true,
      (Some(a), Some(b)) => a.do_eq(b),
      _ => false,
    }
  }
}
impl Eq for MaybeState {}

impl Hash for MaybeState {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    match &self.0 {
      Some(v) => {
        // TODO better hash DynEq object, acceptable for now
        state.write_usize(Rc::as_ptr(v) as *const () as usize);
      }
      None => 0.hash(state),
    }
  }
}

impl MaybeState {
  pub fn new(state: Option<Rc<dyn DynEq>>) -> Self {
    Self(state)
  }

  pub fn cast_or_default<T>(&self) -> Result<Rc<T>, String>
  where
    T: Clone + Default + 'static,
  {
    match &self.0 {
      Some(v) => match v.as_ref().as_any().downcast_ref::<T>() {
        Some(v) => Ok(Rc::new(v.clone())),
        None => Err(format!("failed to cast state to {}", std::any::type_name::<T>())),
      },
      None => Ok(Rc::new(T::default())),
    }
  }
}
