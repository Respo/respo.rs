use std::boxed::Box;
use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use serde_json::Value;
use web_sys::{InputEvent, KeyboardEvent, MouseEvent, Node};

use super::css::RespoStyle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespoNode<T>
where
  T: Debug + Clone,
{
  Component(String, Vec<RespoEffect>, Box<RespoNode<T>>),
  Element {
    /// tagName
    name: String,
    attrs: HashMap<String, String>,
    event: HashMap<String, RespoEventHandler<T>>,
    style: RespoStyle,
    children: Vec<(RespoIndexKey, RespoNode<T>)>,
  },
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RespoIndexKey(String);

impl<T> From<T> for RespoIndexKey
where
  T: Display + Clone + Debug,
{
  fn from(data: T) -> Self {
    Self(data.to_string())
  }
}

// impl Display for RespoIndexKey {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{}", self.0)
//   }
// }

impl<T> RespoNode<T>
where
  T: Debug + Clone,
{
  pub fn make_tag(name: &str) -> Self {
    Self::Element {
      name: name.to_owned(),
      attrs: HashMap::new(),
      event: HashMap::new(),
      style: RespoStyle::default(),
      children: Vec::new(),
    }
  }

  pub fn add_style(&mut self, more: RespoStyle) -> &mut Self {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_style(more);
      }
      RespoNode::Element { ref mut style, .. } => {
        for (k, v) in more.0.into_iter() {
          style.0.push((k.to_owned(), v.to_owned()));
        }
      }
    }
    self
  }
  pub fn add_attrs<U, V, W>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (V, W)>,
    V: Into<String> + ToOwned,
    W: Into<String> + ToOwned,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_attrs(more);
      }
      RespoNode::Element { ref mut attrs, .. } => {
        for (k, v) in more {
          attrs.insert(k.into(), v.into());
        }
      }
    }
    self
  }
  pub fn add_event<U, V>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (V, RespoEventHandler<T>)>,
    V: Into<String> + ToOwned,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_event(more);
      }
      RespoNode::Element { ref mut event, .. } => {
        for (k, v) in more {
          event.insert(k.into(), v.to_owned());
        }
      }
    }
    self
  }
  pub fn add_children<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = RespoNode<T>>,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_children(more);
      }
      RespoNode::Element { ref mut children, .. } => {
        for (idx, v) in more.into_iter().enumerate() {
          children.push((idx.into(), v));
        }
      }
    }
    self
  }

  pub fn add_effects<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = RespoEffect>,
  {
    match self {
      RespoNode::Component(_, ref mut effects, _) => {
        effects.extend(more);
        self
      }
      RespoNode::Element { .. } => unreachable!("effects are on components"),
    }
  }
}

pub type StrDict = HashMap<String, String>;

#[derive(Clone)]
pub struct RespoEventHandler<T>(pub Rc<dyn Fn(RespoEvent, DispatchFn<T>) -> Result<(), String>>)
where
  T: Debug + Clone;

impl<T> PartialEq for RespoEventHandler<T>
where
  T: Debug + Clone,
{
  /// returns true since informations are erased when attaching to the DOM
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<T> Eq for RespoEventHandler<T> where T: Debug + Clone {}

impl<T> Debug for RespoEventHandler<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

#[derive(Debug, Clone)]
pub enum RespoCoord {
  Idx(u32),
  /// for indexing by component name, even though there's only one of that
  Comp(String),
}

/// marks on virtual DOM to declare that there's an event
#[derive(Debug, Clone)]
pub struct RespoEventMark {
  /// location of element in the tree
  pub coord: Vec<RespoCoord>,
  /// TODO event type
  pub name: String,
  /// partial copy of DOM events, that shares across threads,
  /// but being async, methods like `.prevent_default()` and `.stop_propagation()` will not work
  pub event_info: RespoEvent,
}

#[derive(Debug, Clone)]
/// event wraps on top of DOM events
pub enum RespoEvent {
  Click {
    client_x: f64,
    client_y: f64,
    original_event: MouseEvent,
  },
  Keyboard {
    key: String,
    key_code: u32,
    shift_key: bool,
    ctrl_key: bool,
    alt_key: bool,
    meta_key: bool,
    repeat: bool,
    original_event: KeyboardEvent,
  },
  Input {
    value: String,
    original_event: InputEvent,
  },
}

/// TODO need a container for values
#[derive(Debug, Clone)]
pub struct RespoEffect {
  pub args: Vec<Value>,
  pub handler: RespoEffectHandler,
}

impl PartialEq for RespoEffect {
  /// closure are not compared, changes happen in and passed via args
  fn eq(&self, other: &Self) -> bool {
    self.args == other.args
  }
}

impl Eq for RespoEffect {}

impl RespoEffect {
  pub fn run(&self, effect_type: RespoEffectType, el: &Node) -> Result<(), String> {
    (self.handler.0)(self.args.to_owned(), effect_type, el)
  }
}

type UnitStrResult = Result<(), String>;

#[derive(Clone)]
pub struct RespoEffectHandler(pub Rc<dyn Fn(Vec<Value>, RespoEffectType, &Node) -> UnitStrResult>);

impl Debug for RespoEffectHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoEffectType {
  Mounted,
  BeforeUnmount,
  Updated,
  BeforeUpdate,
}

#[derive(Debug, Clone)]
pub enum DomChange<T>
where
  T: Debug + Clone,
{
  ReplaceElement {
    coord: Vec<RespoCoord>,
    node: RespoNode<T>,
  },
  ModifyChildren {
    coord: Vec<RespoCoord>,
    operations: Vec<ChildDomOp<T>>,
  },
  ModifyAttrs {
    coord: Vec<RespoCoord>,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyStyle {
    coord: Vec<RespoCoord>,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyEvent {
    coord: Vec<RespoCoord>,
    add: HashSet<String>,
    remove: HashSet<String>,
  },
  Effect {
    coord: Vec<RespoCoord>,
    effect_type: RespoEffectType,
    // when args not changed in update, that effects are not re-run
    skip_indexes: HashSet<u32>,
  },
}

impl<T> DomChange<T>
where
  T: Debug + Clone,
{
  pub fn get_coord(&self) -> Vec<RespoCoord> {
    match self {
      DomChange::ReplaceElement { coord, .. } => coord.clone(),
      DomChange::ModifyChildren { coord, .. } => coord.clone(),
      DomChange::ModifyAttrs { coord, .. } => coord.clone(),
      DomChange::ModifyStyle { coord, .. } => coord.clone(),
      DomChange::ModifyEvent { coord, .. } => coord.clone(),
      DomChange::Effect { coord, .. } => coord.clone(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum ChildDomOp<T>
where
  T: Debug + Clone,
{
  InsertAfter(u32, RespoNode<T>),
  RemoveAt(u32),
  Append(RespoNode<T>),
}

#[derive(Clone)]
pub struct DispatchFn<T>(pub Rc<dyn Fn(T) -> Result<(), String>>)
where
  T: Debug + Clone;

impl<T> Debug for DispatchFn<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("[DispatchFn]")
  }
}

impl<T> DispatchFn<T>
where
  T: Debug + Clone,
{
  pub fn run(&self, op: T) -> Result<(), String> {
    (self.0)(op)
  }
}

#[derive(Clone)]
pub struct EventHandlerFn(pub Rc<dyn Fn(RespoEventMark) -> Result<(), String>>);

impl Debug for EventHandlerFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("[EventHandlerFn]")
  }
}

impl EventHandlerFn {
  pub fn run(&self, e: RespoEventMark) -> Result<(), String> {
    (self.0)(e)
  }
}
