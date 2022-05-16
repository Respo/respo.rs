use std::fmt::Display;
use std::{collections::HashMap, fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub enum RespoNode {
  Component(String, Vec<RespoEffect>, Box<RespoNode>),
  Element {
    /// tagName
    name: String,
    attrs: HashMap<String, String>,
    event: HashMap<String, RespoEventHandler>,
    style: RespoCssStyle,
    children: Vec<RespoNode>,
  },
}

pub type StrDict = HashMap<String, String>;
#[derive(Clone)]
pub struct RespoEventHandler(pub Rc<dyn Fn() -> Result<(), String>>);

impl Debug for RespoEventHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

#[derive(Debug, Clone)]
pub struct RespoCssStyle(pub HashMap<String, String>);

impl Display for RespoCssStyle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (key, value) in self.0.iter() {
      write!(f, "{}:{};", key, value)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone)]
pub enum RespoCoord {
  Idx(u32),
  /// for indexing by component name, even though there's only one of that
  Comp(String),
}

#[derive(Debug, Clone)]
pub struct RespoEvent {
  /// location of element in the tree
  pub coord: Vec<RespoCoord>,
  /// TODO event type
  pub name: String,
}

#[derive(Debug, Clone)]
pub struct RespoEffect {
  /// TODO need a container for values
  args: Vec<String>,
  handler: RespoEffectHandler,
}

#[derive(Clone)]
pub struct RespoEffectHandler(pub Rc<dyn FnMut() -> Result<(), String>>);

impl Debug for RespoEffectHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}
