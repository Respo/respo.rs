use std::{any::Any, fmt::Debug, rc::Rc};

use cirru_parser::Cirru;
use web_sys::Node;

// use crate::{log, util::print_type_of};

/// effects that attached to components
#[derive(Clone)]
pub struct RespoEffect {
  /// arguments passed to this effect.
  /// the events `WillUpdate` and `Updated` are triggered when these arguments are changed
  pub args: Vec<RespoEffectArg>,
  handler: Rc<RespoEffectHandler>,
}

type RespoEffectHandler = dyn Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String>;

impl PartialEq for RespoEffect {
  /// closure are not compared, changes happen in and passed via args
  fn eq(&self, other: &Self) -> bool {
    self.args == other.args
  }
}

impl Eq for RespoEffect {}

impl RespoEffect {
  pub fn run(&self, effect_type: RespoEffectType, el: &Node) -> Result<(), String> {
    (*self.handler)(self.args.to_owned(), effect_type, el)
  }
  pub fn new<U, V>(args: Vec<V>, handler: U) -> Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
    V: Clone + DynEq + Debug + 'static,
  {
    Self {
      args: args.into_iter().map(RespoEffectArg::new).collect(),
      handler: Rc::new(handler),
    }
  }

  /// no need to have args, only handler
  pub fn new_insular<U>(handler: U) -> Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
  {
    Self {
      args: vec![],
      handler: Rc::new(handler),
    }
  }
}

impl Debug for RespoEffect {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEffect(")?;
    write!(f, "args: {:?}", self.args)?;
    write!(f, "...)")
  }
}

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

/// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c39e1eef6c8c10e973fa629103b4a0b1
pub trait DynEq: Debug {
  fn as_any(&self) -> &dyn Any;
  fn do_eq(&self, rhs: &dyn DynEq) -> bool;
}

impl<T> DynEq for T
where
  T: PartialEq + Debug + 'static,
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
