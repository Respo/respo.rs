mod dom_change;

use std::boxed::Box;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use cirru_parser::Cirru;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{FocusEvent, InputEvent, KeyboardEvent, MouseEvent, Node};

use crate::{MaybeState, StatesTree};

use super::css::RespoStyle;

pub use dom_change::{changes_to_cirru, ChildDomOp, DomChange, RespoCoord};

/// an `Element` or a `Component`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespoNode<T>
where
  T: Debug + Clone,
{
  Component(String, Vec<RespoEffect>, Box<RespoNode<T>>),
  /// corresponding to DOM elements
  Element {
    /// tagName
    name: String,
    attrs: HashMap<String, String>,
    event: HashMap<String, RespoListenerFn<T>>,
    /// inlines styles, partially typed.
    /// there's also a macro called `static_styles` for inserting CSS rules
    style: RespoStyle,
    /// each child as a key like a string, by default generated from index,
    /// they are used in diffing, so it's better to be distinct, although not required to be.
    children: Vec<(RespoIndexKey, RespoNode<T>)>,
  },
  Referenced(Rc<RespoNode<T>>),
}

impl<T> From<RespoNode<T>> for Cirru
where
  T: Debug + Clone,
{
  fn from(value: RespoNode<T>) -> Self {
    match value {
      RespoNode::Component(name, _eff, tree) => {
        Cirru::List(vec![Cirru::Leaf("::Component".into()), Cirru::Leaf(name.into()), (*tree).into()])
      }
      RespoNode::Element { name, children, .. } => {
        let mut xs = vec![Cirru::Leaf(name.into())];
        for (k, child) in children {
          xs.push(Cirru::List(vec![Cirru::Leaf(k.to_string().into()), child.to_owned().into()]));
        }
        Cirru::List(xs)
      }
      RespoNode::Referenced(cell) => (*cell).to_owned().into(),
    }
  }
}

impl<T> Display for RespoNode<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

/// a key for referencing a child node, use a value that can be converted to string
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct RespoIndexKey(String);

impl From<usize> for RespoIndexKey {
  fn from(data: usize) -> Self {
    Self(data.to_string())
  }
}

impl From<String> for RespoIndexKey {
  fn from(s: String) -> Self {
    Self(s)
  }
}

impl From<&str> for RespoIndexKey {
  fn from(s: &str) -> Self {
    Self(s.to_owned())
  }
}

impl Display for RespoIndexKey {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<RespoIndexKey> for Cirru {
  fn from(k: RespoIndexKey) -> Cirru {
    k.to_string().into()
  }
}

impl<T> RespoNode<T>
where
  T: Debug + Clone,
{
  /// create an element node
  pub fn new_tag(name: &str) -> Self {
    Self::Element {
      name: name.to_owned(),
      attrs: HashMap::new(),
      event: HashMap::new(),
      style: RespoStyle::default(),
      children: Vec::new(),
    }
  }
  /// create a new component
  pub fn new_component(name: &str, tree: RespoNode<T>) -> Self {
    Self::Component(name.to_owned(), Vec::new(), Box::new(tree))
  }
  /// attach styles
  /// ```ignore
  /// element.style(RespoStyle::default().margin(10))
  /// ```
  pub fn style(&mut self, more: RespoStyle) -> &mut Self {
    match self {
      RespoNode::Component(_, _, node) => {
        node.style(more);
      }
      RespoNode::Element { ref mut style, .. } => {
        for (k, v) in more.0.into_iter() {
          style.0.push((k.to_owned(), v.to_owned()));
        }
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// imparative way of updating style
  /// ```ignore
  /// element.modify_style(|s| {
  ///   if data > 1 {
  ///     s.color(CssColor::Red);
  ///   }
  /// });
  /// ```
  pub fn modify_style<U>(&mut self, builder: U) -> &mut Self
  where
    U: Fn(&mut RespoStyle),
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.modify_style(builder);
      }
      RespoNode::Element { ref mut style, .. } => {
        builder(style);
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// set an attribute on element
  pub fn attribute<U, V>(&mut self, property: U, value: V) -> &mut Self
  where
    U: Into<String> + ToOwned,
    V: Display,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.attribute(property, value);
      }
      RespoNode::Element { ref mut attrs, .. } => {
        attrs.insert(property.into(), value.to_string());
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// set an attribute on element, but using `None` indicates noting
  pub fn maybe_attribute<U, V>(&mut self, property: U, value: Option<V>) -> &mut Self
  where
    U: Into<String> + ToOwned,
    V: Display,
  {
    if let Some(v) = value {
      match self {
        RespoNode::Component(_, _, node) => {
          node.attribute(property, v);
        }
        RespoNode::Element { ref mut attrs, .. } => {
          attrs.insert(property.into(), v.to_string());
        }
        RespoNode::Referenced(_) => {
          unreachable!("should not be called on a referenced node");
        }
      }
    }
    self
  }
  pub fn on_click<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("click", handler);
    self
  }
  pub fn on_input<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("input", handler);
    self
  }
  /// handle keydown event
  pub fn on_keydown<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("keydown", handler);
    self
  }
  /// handle focus event
  pub fn on_focus<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("focus", handler);
    self
  }
  /// handle change event
  pub fn on_change<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("change", handler);
    self
  }
  /// attach a listener by event name(only a small set of events are supported)
  pub fn on_named_event<U>(&mut self, name: &str, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.on_named_event(name, handler);
      }
      RespoNode::Element { ref mut event, .. } => {
        event.insert(name.into(), RespoListenerFn::new(handler));
      }
      RespoNode::Referenced(_) => {
        unreachable!("should attach event on a referenced node");
      }
    }
    self
  }
  /// add children elements,
  /// index key are generated from index number
  pub fn children<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = RespoNode<T>>,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.children(more);
      }
      RespoNode::Element { ref mut children, .. } => {
        for (idx, v) in more.into_iter().enumerate() {
          children.push((idx.into(), v));
        }
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// add children elements, with index keys specified
  pub fn children_indexed<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (RespoIndexKey, RespoNode<T>)>,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.children_indexed(more);
      }
      RespoNode::Element { ref mut children, .. } => {
        for (idx, v) in more {
          children.push((idx, v));
        }
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// add an effect on component
  pub fn effect<U, V>(&mut self, args: &[V], handler: U) -> &mut Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
    V: Serialize + Clone,
  {
    match self {
      RespoNode::Component(_, ref mut effects, _) => {
        effects.push(RespoEffect::new(args.to_vec(), handler));
        self
      }
      RespoNode::Element { .. } => unreachable!("effects are on components"),
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
  }
  /// add an empty args effect on component, which does not update
  pub fn stable_effect<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
  {
    match self {
      RespoNode::Component(_, ref mut effects, _) => {
        effects.push(RespoEffect::new(vec![] as Vec<()>, handler));
        self
      }
      RespoNode::Element { .. } => unreachable!("effects are on components"),
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
  }
  /// add a list of effects on component
  pub fn effects<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = RespoEffect>,
  {
    match self {
      RespoNode::Component(_, ref mut effects, _) => {
        effects.extend(more);
        self
      }
      RespoNode::Element { .. } => unreachable!("effects are on components"),
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
  }
  /// attach a class name for adding styles
  pub fn class<U>(&mut self, name: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.attribute("class", name.into())
  }
  /// attach an optional class name for adding styles
  pub fn maybe_class<U>(&mut self, name: Option<U>) -> &mut Self
  where
    U: Into<String>,
  {
    match name {
      Some(name) => self.attribute("class", name.into()),
      None => self,
    }
  }
  /// attach a class name, controlled by a boolean
  pub fn toggle_class<U>(&mut self, name: U, on: bool) -> &mut Self
  where
    U: Into<String>,
  {
    if on {
      self.attribute("class", name.into());
    }
    self
  }
  /// attach a list of class names for adding styles
  pub fn class_list<U>(&mut self, names: &[U]) -> &mut Self
  where
    U: Into<String> + Clone,
  {
    let mut class_name: Vec<String> = vec![];
    for name in names {
      class_name.push((*name).to_owned().into());
    }
    self.attribute("class", class_name.join(" "));
    self
  }
  /// writes `innerText`
  pub fn inner_text<U>(&mut self, content: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.attribute("innerText", content.into());
    self
  }
  /// writes `innerHTML`
  pub fn inner_html<U>(&mut self, content: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.attribute("innerHTML", content.into());
    self
  }
  /// writes `value`
  pub fn value<U>(&mut self, content: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.attribute("value", content.into());
    self
  }
  /// wrap with a `Rc<RefCell<T>>` to enable memory reuse and skipping in diff
  pub fn share_with_ref(&self) -> Self {
    Self::Referenced(Rc::new(self.clone()))
  }
}

pub(crate) type StrDict = HashMap<String, String>;

fn str_dict_to_cirrus_dict(dict: &StrDict) -> Cirru {
  let mut xs = vec![];
  for (k, v) in dict {
    xs.push(vec![k.to_owned(), v.to_owned()].into());
  }
  Cirru::List(xs)
}

/// (internal) struct to store event handler function on the tree
#[derive(Clone)]
pub struct RespoListenerFn<T>(Rc<dyn Fn(RespoEvent, DispatchFn<T>) -> Result<(), String>>)
where
  T: Debug + Clone;

impl<T> PartialEq for RespoListenerFn<T>
where
  T: Debug + Clone,
{
  /// returns true since informations are erased when attaching to the DOM
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<T> Eq for RespoListenerFn<T> where T: Debug + Clone {}

impl<T> Debug for RespoListenerFn<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

impl<T> RespoListenerFn<T>
where
  T: Debug + Clone,
{
  pub fn new<U>(handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    Self(Rc::new(handler))
  }
  pub fn run(&self, event: RespoEvent, dispatch: DispatchFn<T>) -> Result<(), String> {
    (self.0)(event, dispatch)
  }
}

/// marks on virtual DOM to declare that there's an event
/// event handler is HIDDEN from this mark.
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

impl RespoEventMark {
  pub fn new(name: &str, coord: &[RespoCoord], event: RespoEvent) -> Self {
    Self {
      name: name.to_owned(),
      coord: coord.to_vec(),
      event_info: event,
    }
  }
}

/// event wraps on top of DOM events
#[derive(Debug, Clone)]
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
  Focus(FocusEvent),
  Blur(FocusEvent),
}

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

/// dispatch function passed from root of renderer,
/// call it like `dispatch.run(op)`
#[derive(Clone)]
pub struct DispatchFn<T>(Rc<dyn Fn(T) -> Result<(), String>>)
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

/// it has special support for states
pub trait RespoAction {
  /// to provide syntax sugar to dispatch.run_state
  fn wrap_states_action(cursor: &[String], a: MaybeState) -> Self;
}

impl<T> DispatchFn<T>
where
  T: Debug + Clone + RespoAction,
{
  /// dispatch an action
  pub fn run(&self, op: T) -> Result<(), String> {
    (self.0)(op)
  }
  /// dispatch to update local state
  pub fn run_state<U>(&self, cursor: &[String], data: U) -> Result<(), String>
  where
    U: Serialize,
  {
    (self.0)(T::wrap_states_action(
      cursor,
      MaybeState::new(Some(serde_json::to_value(data).map_err(|e| e.to_string())?)),
    ))
  }
  /// reset state to empty
  pub fn run_empty_state(&self, cursor: &[String]) -> Result<(), String> {
    (self.0)(T::wrap_states_action(cursor, MaybeState::new(None)))
  }
  pub fn new<U>(f: U) -> Self
  where
    U: Fn(T) -> Result<(), String> + 'static,
  {
    Self(Rc::new(f))
  }
}

/// (internal) function to handle event marks at first phase of event handling
#[derive(Clone)]
pub struct RespoEventMarkFn(Rc<dyn Fn(RespoEventMark) -> Result<(), String>>);

impl Debug for RespoEventMarkFn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("[EventMarkFn ...]")
  }
}

impl RespoEventMarkFn {
  pub fn run(&self, e: RespoEventMark) -> Result<(), String> {
    (self.0)(e)
  }
  pub fn new<U>(f: U) -> Self
  where
    U: Fn(RespoEventMark) -> Result<(), String> + 'static,
  {
    Self(Rc::new(f))
  }
}

impl From<Rc<dyn Fn(RespoEventMark) -> Result<(), String>>> for RespoEventMarkFn {
  fn from(f: Rc<dyn Fn(RespoEventMark) -> Result<(), String>>) -> Self {
    Self(f)
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

/// it has a states tree inside, and it does update itself
pub trait RespoStore {
  type Action: Debug + Clone + RespoAction;
  fn get_states(&self) -> StatesTree;
  fn update(&mut self, op: Self::Action) -> Result<(), String>;
}
