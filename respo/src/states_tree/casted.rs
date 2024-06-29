use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

use super::{RespoState, RespoStatesTree};

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

impl<T> Hash for RespoStatesTreeCasted<T>
where
  T: Hash,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.cursor.hash(state);
    self.data.hash(state);
    // backup is not real data
    self.branches.hash(state);
  }
}

impl<T> PartialEq for RespoStatesTreeCasted<T>
where
  T: PartialEq,
{
  fn eq(&self, other: &Self) -> bool {
    // backup is only for backup
    // this trick might cause inconsistency in some cases after reloaded
    self.cursor == other.cursor && self.data == other.data && self.branches == other.branches
  }
}
impl<T> Eq for RespoStatesTreeCasted<T> where T: PartialEq {}

impl<T> RespoStatesTreeCasted<T>
where
  T: Default,
{
  /// get cursor
  pub fn path(&self) -> Vec<Rc<str>> {
    self.cursor.to_owned()
  }

  /// pick a child branch as new cursor
  pub fn pick_to<U>(&self, name: &str) -> Result<RespoStatesTreeCasted<U>, String>
  where
    U: Clone + Default + RespoState + 'static,
  {
    let mut next_cursor = self.cursor.to_owned();
    next_cursor.push(Rc::from(name));

    if self.branches.contains_key(name) {
      let prev = &self.branches[name];
      let branch = prev.cast_branch::<U>()?;
      Ok(RespoStatesTreeCasted {
        data: branch.to_owned(),
        backup: prev.backup.to_owned(),
        // data_revision: prev.data_revision,
        // data_type_name: prev.data_type_name.to_owned(),
        cursor: next_cursor,
        branches: prev.branches.to_owned(),
      })
    } else {
      Ok(RespoStatesTreeCasted {
        data: Rc::new(U::default()),
        backup: None,
        cursor: next_cursor,
        // data_type_name: None,
        // data_revision: 0,
        branches: BTreeMap::new(),
      })
    }
  }
}
