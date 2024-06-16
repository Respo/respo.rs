mod base;

use std::{any::Any, fmt::Debug, rc::Rc};

use base::{AsRespoEffectBase, RespoEffectDynEq};
use cirru_parser::Cirru;
use web_sys::Node;

/// trait for component effects
/// you can declare `mounted`, `beforeUpdate`, `updated`, `beforeUnmount` methods
/// to handle lifecycle events, mainly for manually manipulating DOM
pub trait RespoEffect
where
  Self: Debug + Any + RespoEffectDynEq + AsRespoEffectBase + 'static,
{
  /// actually run effect
  #[allow(unused_variables)]
  fn run(&self, effect_type: RespoEffectType, el: &Node) -> Result<(), String> {
    match effect_type {
      RespoEffectType::Mounted => self.mounted(el),
      RespoEffectType::BeforeUpdate => self.before_update(el),
      RespoEffectType::Updated => self.updated(el),
      RespoEffectType::BeforeUnmount => self.before_unmount(el),
    }
  }
  /// called when mounted
  #[allow(unused_variables)]
  fn mounted(&self, el: &Node) -> Result<(), String> {
    Ok(())
  }
  /// called when before update
  #[allow(unused_variables)]
  fn before_update(&self, el: &Node) -> Result<(), String> {
    Ok(())
  }
  /// called when updated
  #[allow(unused_variables)]
  fn updated(&self, el: &Node) -> Result<(), String> {
    Ok(())
  }
  /// called when before unmount
  #[allow(unused_variables)]
  fn before_unmount(&self, el: &Node) -> Result<(), String> {
    Ok(())
  }
}

/// wraps dyn trait object of effect
#[derive(Debug, Clone)]
pub struct RespoEffectBox(pub Rc<dyn RespoEffect>);

impl PartialEq for RespoEffectBox {
  fn eq(&self, other: &Self) -> bool {
    let r = self.0.as_ref();
    r.do_eq(other.0.as_ref().as_base()) == Some(true)
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

/// Internal enum for effect types.
/// you only need this if you override `RespoEffect` `.run()`.
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
