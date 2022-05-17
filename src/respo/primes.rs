use std::boxed::Box;
use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

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
    digit_coord: DigitCoord,
    respo_coord: Vec<RespoCoord>,
    client_x: f64,
    client_y: f64,
  },
  Keyboard {
    digit_coord: DigitCoord,
    respo_coord: Vec<RespoCoord>,
    key: String,
    key_code: u32,
    shift_key: bool,
    ctrl_key: bool,
    alt_key: bool,
    meta_key: bool,
    repeat: bool,
  },
  Input {
    digit_coord: DigitCoord,
    respo_coord: Vec<RespoCoord>,
    value: String,
  },
}

/// TODO need a container for values
#[derive(Debug, Clone)]
pub struct RespoEffect {
  // args: Vec<String>,
// handler: RespoEffectHandler,
}

#[derive(Clone)]
pub struct RespoEffectHandler(pub Rc<dyn FnMut() -> Result<(), String>>);

impl Debug for RespoEffectHandler {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

#[derive(Debug, Clone, Default)]
pub struct DigitCoord(pub Vec<u32>);

impl<T> From<T> for DigitCoord
where
  T: IntoIterator<Item = RespoCoord>,
{
  fn from(coord: T) -> Self {
    let mut res = Vec::new();
    for c in coord {
      match c {
        RespoCoord::Idx(idx) => res.push(idx),
        // ignore component paths
        RespoCoord::Comp(..) => {}
      }
    }
    Self(res)
  }
}

impl DigitCoord {
  pub fn extend(&self, idx: u32) -> Self {
    let mut res = self.0.clone();
    res.push(idx);
    Self(res)
  }
}

#[derive(Debug, Clone)]
pub enum DomChange<T>
where
  T: Debug + Clone,
{
  ReplaceElement {
    digit_coord: DigitCoord,
    node: RespoNode<T>,
  },
  ModifyChildren {
    digit_coord: DigitCoord,
    operations: Vec<ChildDomOp<T>>,
  },
  ModifyAttrs {
    digit_coord: DigitCoord,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyStyle {
    digit_coord: DigitCoord,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyEvent {
    digit_coord: DigitCoord,
    respo_coord: Vec<RespoCoord>,
    add: HashSet<String>,
    remove: HashSet<String>,
  }, // TODO effects not started
}

impl<T> DomChange<T>
where
  T: Debug + Clone,
{
  pub fn get_coord(&self) -> DigitCoord {
    match self {
      DomChange::ReplaceElement { digit_coord, .. } => digit_coord.clone(),
      DomChange::ModifyChildren { digit_coord, .. } => digit_coord.clone(),
      DomChange::ModifyAttrs { digit_coord, .. } => digit_coord.clone(),
      DomChange::ModifyStyle { digit_coord, .. } => digit_coord.clone(),
      DomChange::ModifyEvent { digit_coord, .. } => digit_coord.clone(),
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
