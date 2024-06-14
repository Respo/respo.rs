use std::{any::Any, fmt::Debug, rc::Rc};

use cirru_parser::Cirru;
use web_sys::Node;

use crate::states_tree::DynEq;

/// next abstraction on effect
pub trait RespoEffect
where
  Self: Debug + 'static,
{
  /// actually run effect
  fn run(&self, effect_type: RespoEffectType, el: &Node) -> Result<(), String>;
  fn as_any(&self) -> &dyn Any;
  fn do_eq(&self, rhs: &dyn RespoEffect) -> bool;
}

/// wraps dyn trait object of effect
#[derive(Debug, Clone)]
pub struct RespoEffectBox(pub Rc<dyn RespoEffect>);

impl PartialEq for RespoEffectBox {
  fn eq(&self, other: &Self) -> bool {
    let r = self.0.as_ref();
    r.do_eq(other.0.as_ref())
  }
}
impl Eq for RespoEffectBox {}

impl RespoEffectBox {
  pub fn new<T>(v: T) -> Self
  where
    T: RespoEffect + 'static,
  {
    Self(Rc::new(v))
  }
}

// use crate::{log, util::print_type_of};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoEffectType {
  /// called after mounting happened, use effect handlers from new trees
  Mounted,
  /// called before effect arguments changed, use effect hanles from new trees
  BeforeUpdate,
  /// called after effect arguments changed, use effect handles from new trees
  Updated,
  /// called before unmounting, use effect handles from **old** trees
  BeforeUnmount,
}

impl From<RespoEffectType> for Cirru {
  fn from(effect_type: RespoEffectType) -> Self {
    match effect_type {
      RespoEffectType::Mounted => "::mounted".into(),
      RespoEffectType::BeforeUpdate => "::before-update".into(),
      RespoEffectType::Updated => "::updated".into(),
      RespoEffectType::BeforeUnmount => "::before-unmount".into(),
    }
  }
}

/// (internal) abstraction on effect argument
#[derive(Debug, Clone)]
pub struct RespoEffectArg(pub Rc<dyn DynEq>);

impl PartialEq for RespoEffectArg {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl Eq for RespoEffectArg {}

impl RespoEffectArg {
  pub fn new<T: ToOwned + DynEq + 'static>(v: T) -> Self {
    Self(Rc::new(v))
  }
  pub fn cast_into<T>(&self) -> Result<T, String>
  where
    T: Debug + DynEq + Clone + 'static,
  {
    // log!("cast {:?} {:?}", self.0, type_name::<T>());
    // print_type_of(&self.0.as_ref());
    // log!("expected type {:?}", type_name::<T>());
    // if let Some(v) = self.0.as_ref().as_any().downcast_ref::<bool>() {
    //   log!("Casted to &bool {:?}", v);
    // } else {
    //   log!("failed to cast &bool {:?}", self.0);
    // }
    if let Some(v) = self.0.as_ref().as_any().downcast_ref::<T>() {
      // need to call .as_ref() to get the reference inside Rc<T>
      Ok(v.to_owned())
    } else {
      Err(format!("failed to cast, {:?}", self))
    }
  }
}
