use std::{any::Any, fmt::Debug};

use super::RespoState;

/// <https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c39e1eef6c8c10e973fa629103b4a0b1>
pub trait DynEq: Debug + RespoState {
  fn as_any(&self) -> &dyn Any;
  fn do_eq(&self, rhs: &dyn DynEq) -> bool;
}

impl<T> DynEq for T
where
  T: PartialEq + Debug + RespoState + 'static,
{
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn do_eq(&self, rhs: &dyn DynEq) -> bool {
    if let Some(rhs_concrete) = rhs.as_any().downcast_ref::<Self>() {
      self == rhs_concrete
    } else {
      false
    }
  }
}

impl PartialEq for dyn DynEq {
  fn eq(&self, rhs: &Self) -> bool {
    self.do_eq(rhs)
  }
}
