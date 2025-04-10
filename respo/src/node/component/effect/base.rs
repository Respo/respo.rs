//! in order to make Effect implement PartialEq and as Any, some tricks are needed

use std::{any::Any, fmt::Debug};


pub trait RespoEffectDynEq
where
  Self: Debug + Any + 'static,
{
  fn as_any(&self) -> &dyn Any;
  fn do_eq(&self, rhs: &dyn RespoEffectDynEq) -> Option<bool>;
}

impl<T> RespoEffectDynEq for T
where
  T: PartialEq + Debug + 'static,
{
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn do_eq(&self, rhs: &dyn RespoEffectDynEq) -> Option<bool> {
    if let Some(rhs_concrete) = rhs.as_any().downcast_ref::<Self>() {
      Some(self == rhs_concrete)
    } else {
      Some(false)
    }
  }
}

impl PartialEq for dyn RespoEffectDynEq {
  fn eq(&self, rhs: &Self) -> bool {
    self.do_eq(rhs) == Some(true)
  }
}
