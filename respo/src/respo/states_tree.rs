use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::hash::Hash;
use std::{collections::HashMap, rc::Rc};

// use wasm_bindgen::JsValue;
// use web_sys::console::log_1;

/// Respo maintains states in a tree structure, where the keys are strings,
/// each child component "picks" a key to attach its own state to the tree,
/// and it dispatches events to global store to update the state.
#[derive(Debug, Clone, Default)]
pub struct StatesTree {
  /// local data
  pub data: MaybeState,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<String>,
  pub data_type_name: Option<TypeId>,
  pub data_revision: usize,
  /// holding children states
  pub branches: HashMap<String, Box<StatesTree>>,
}

impl Hash for StatesTree {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.cursor.hash(state);
    self.data_type_name.hash(state);
    self.data_revision.hash(state);
  }
}

impl PartialEq for StatesTree {
  fn eq(&self, other: &Self) -> bool {
    // data and revision to simulate state change
    self.cursor == other.cursor && self.data_type_name == other.data_type_name && self.data_revision == other.data_revision
  }
}

impl Eq for StatesTree {}

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
        data_revision: prev.data_revision,
        data_type_name: prev.data_type_name.to_owned(),
        cursor: next_cursor,
        branches: prev.branches.clone(),
      }
    } else {
      Self {
        data: MaybeState::new(None),
        cursor: next_cursor,
        data_type_name: None,
        data_revision: 0,
        branches: HashMap::new(),
      }
    }
  }

  /// in-place mutation of state tree
  pub fn set_in_mut(&mut self, path: &[String], new_state: MaybeState) {
    if path.is_empty() {
      new_state.clone_into(&mut self.data);
      self.data_type_name = new_state.0.as_ref().map(|v| v.type_id().to_owned());
      self.data_revision += 1;
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
pub struct MaybeState(Option<Rc<dyn Any>>);

impl MaybeState {
  pub fn new(state: Option<Rc<dyn Any>>) -> Self {
    Self(state)
  }

  pub fn cast_or_default<T>(&self) -> Result<Rc<T>, String>
  where
    T: Clone + Default + 'static,
  {
    match &self.0 {
      Some(v) => match v.downcast_ref::<T>() {
        Some(v) => Ok(Rc::new(v.clone())),
        None => Err(format!("failed to cast state to {}", std::any::type_name::<T>())),
      },
      None => Ok(Rc::new(T::default())),
    }
  }
}
