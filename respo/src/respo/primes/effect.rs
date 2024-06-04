use std::{fmt::Debug, rc::Rc};

use cirru_parser::Cirru;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use web_sys::Node;

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
    V: Serialize,
  {
    Self {
      args: args
        .iter()
        .map(|v| RespoEffectArg::new(serde_json::to_value(v).expect("to json")))
        .collect(),
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

/// (internal) abstraction on effect argument
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RespoEffectArg(Value);

impl RespoEffectArg {
  pub fn new(v: Value) -> Self {
    Self(v)
  }
  pub fn cast_into<U>(&self) -> Result<U, String>
  where
    U: DeserializeOwned,
  {
    serde_json::from_value(self.0.clone()).map_err(|e| e.to_string())
  }
}
