use std::boxed::Box;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};
use web_sys::{InputEvent, KeyboardEvent, MouseEvent};

#[derive(Debug, Clone)]
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
    style: RespoCssStyle,
    children: Vec<RespoNode<T>>,
  },
}

impl<T> RespoNode<T>
where
  T: Debug + Clone,
{
  pub fn add_style(&mut self, more: RespoCssStyle) -> Result<&mut Self, String> {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_style(more)?;
      }
      RespoNode::Element { ref mut style, .. } => {
        for (k, v) in &more.0 {
          style.0.insert(k.to_owned(), v.to_owned());
        }
      }
    }
    Ok(self)
  }
  pub fn add_attrs<U, V>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (V, V)>,
    V: Into<String> + ToOwned,
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
        for v in more {
          children.push(v.to_owned());
        }
      }
    }
    self
  }
}

pub type StrDict = HashMap<String, String>;

#[derive(Clone)]
pub struct RespoEventHandler<T>(pub Rc<dyn Fn(RespoEvent, DispatchFn<T>) -> Result<(), String>>)
where
  T: Debug + Clone;

impl<T> Debug for RespoEventHandler<T>
where
  T: Debug + Clone,
{
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
  // TODO
  Click {
    coord: DigitCoord,
    client_x: f64,
    client_y: f64,
  },
  Keyboard {
    coord: DigitCoord,
    key: String,
    key_code: u32,
    shift_key: bool,
    ctrl_key: bool,
    alt_key: bool,
    meta_key: bool,
    repeat: bool,
  },
  Input {
    coord: DigitCoord,
    value: String,
  },
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

#[derive(Debug, Clone)]
pub struct DigitCoord(Vec<u32>);

impl<T> From<T> for DigitCoord
where
  T: IntoIterator<Item = RespoCoord>,
{
  fn from(coord: T) -> Self {
    let mut res = Vec::new();
    for c in coord {
      match c {
        RespoCoord::Idx(idx) => res.push(idx),
        RespoCoord::Comp(name) => {
          // ignore
        }
      }
    }
    Self(res)
  }
}

#[derive(Debug, Clone)]
pub enum DomChange<T>
where
  T: Debug + Clone,
{
  AddElement(DigitCoord, RespoNode<T>),
  AppendElement(DigitCoord, RespoNode<T>),
  RemoveElement(DigitCoord),
  ReplaceElement(DigitCoord, RespoNode<T>),
  AddAttribute(DigitCoord, String, String),
  RemoveAttribute(DigitCoord, String),
  ReplaceAttribute(DigitCoord, String, String),
  AddStyle(DigitCoord, String, String),
  RemoveStyle(DigitCoord, String),
  ReplaceStyle(DigitCoord, String, String),
  AddEvent(DigitCoord, String),
  RemoveEvent(DigitCoord, String),
  // TODO effects not started
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
