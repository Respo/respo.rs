mod dom_change;
mod effect;
mod listener;

use std::boxed::Box;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use cirru_parser::Cirru;
pub use effect::DynEq;
pub use effect::RespoEffectArg;
pub use listener::{RespoEvent, RespoEventMark, RespoListenerFn};
use web_sys::Node;

use crate::{MaybeState, StatesTree};

use super::css::RespoStyle;

pub use dom_change::{changes_to_cirru, ChildDomOp, DomChange, RespoCoord};

pub use effect::{RespoEffect, RespoEffectType};

/// an `Element` or a `Component`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespoNode<T>
where
  T: Debug + Clone,
{
  Component(Rc<str>, Vec<RespoEffect>, Box<RespoNode<T>>),
  /// corresponding to DOM elements
  Element {
    /// tagName
    name: Rc<str>,
    attrs: HashMap<Rc<str>, String>,
    event: HashMap<Rc<str>, RespoListenerFn<T>>,
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
        Cirru::List(vec![Cirru::Leaf("::Component".into()), Cirru::from(name.as_ref()), (*tree).into()])
      }
      RespoNode::Element { name, children, .. } => {
        let mut xs = vec![Cirru::from(name.as_ref())];
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

impl From<&usize> for RespoIndexKey {
  fn from(data: &usize) -> Self {
    Self(data.to_string())
  }
}

impl From<String> for RespoIndexKey {
  fn from(s: String) -> Self {
    Self(s)
  }
}

impl From<&String> for RespoIndexKey {
  fn from(s: &String) -> Self {
    Self(s.to_owned())
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
    Cirru::from(k.to_string())
  }
}

impl<T> RespoNode<T>
where
  T: Debug + Clone,
{
  /// create an element node
  pub fn new_tag(name: &str) -> Self {
    Self::Element {
      name: name.into(),
      attrs: HashMap::new(),
      event: HashMap::new(),
      style: RespoStyle::default(),
      children: Vec::new(),
    }
  }

  /// finish building
  pub fn end(&self) -> Self {
    self.to_owned()
  }
  /// create a new component
  pub fn new_component(name: &str, tree: RespoNode<T>) -> Self {
    Self::Component(name.into(), Vec::new(), Box::new(tree))
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
    U: Into<Rc<str>> + ToOwned,
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
    U: Into<Rc<str>> + ToOwned,
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
          children.push((idx.into(), v.to_owned()));
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
    V: Clone + DynEq + Debug + 'static,
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
    Self::Referenced(Rc::new(self.to_owned()))
  }
}

pub(crate) type StrDict = HashMap<Rc<str>, String>;

fn str_dict_to_cirrus_dict(dict: &StrDict) -> Cirru {
  let mut xs = vec![];
  for (k, v) in dict {
    xs.push(vec![Cirru::from(k.as_ref()), Cirru::from(v)].into());
  }
  Cirru::List(xs)
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
  fn wrap_states_action(cursor: &[Rc<str>], a: MaybeState) -> Self;
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
  pub fn run_state<U>(&self, cursor: &[Rc<str>], data: U) -> Result<(), String>
  where
    U: DynEq + ToOwned + Clone + PartialEq + Eq + 'static,
  {
    let a = Rc::new(data);
    (self.0)(T::wrap_states_action(cursor, MaybeState::new(Some(a))))
  }
  /// reset state to empty
  pub fn run_empty_state(&self, cursor: &[Rc<str>]) -> Result<(), String> {
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

/// it has a states tree inside, and it does update itself
pub trait RespoStore {
  type Action: Debug + Clone + RespoAction;
  fn get_states(&self) -> StatesTree;
  fn update(&mut self, op: Self::Action) -> Result<(), String>;
}
