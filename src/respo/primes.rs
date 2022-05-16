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
}

#[derive(Debug, Clone)]
/// event wraps on top of DOM events
pub enum RespoEvent {
  // TODO
  Click,
  Keyboard(KeyboardEvent),
  Input(InputEvent),
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

pub type DigitCoord = Vec<u32>;

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
