use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::rc::Rc;

use super::RespoStatesTree;

/// similar to RespoStatesTree but with a generic data type
#[derive(Debug, Clone, Default)]
pub struct RespoStatesTreeCasted<T> {
  // #[serde(skip)]
  /// local data
  pub data: Rc<T>,
  pub backup: Option<Value>,
  /// the path to the current state in the tree, use in updating
  pub cursor: Vec<Rc<str>>,
  // pub data_type_name: Option<TypeId>,
  // pub data_revision: usize,
  /// holding children states
  pub branches: BTreeMap<Rc<str>, Box<RespoStatesTree>>,
}

impl<T> RespoStatesTreeCasted<T> {
  /// get cursor
  pub fn path(&self) -> Vec<Rc<str>> {
    self.cursor.to_owned()
  }
}
