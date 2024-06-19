//! RespoNode abstraction

pub(crate) mod component;
pub mod css;
pub(crate) mod dom_change;
pub(crate) mod element;
mod listener;

use std::boxed::Box;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use cirru_parser::Cirru;
pub use listener::RespoEvent;
pub(crate) use listener::{RespoEventMark, RespoListenerFn};

pub use component::RespoComponent;
pub use element::RespoElement;

use crate::states_tree::{DynEq, RespoStateBranch, RespoUpdateState};

use css::RespoStyle;

pub(crate) use dom_change::RespoCoord;
pub(crate) use dom_change::{ChildDomOp, DomChange};

pub use component::effect::{RespoEffect, RespoEffectType};

/// an `Element` or a `Component`
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespoNode<T>
where
  T: Debug + Clone,
{
  Component(RespoComponent<T>),
  /// corresponding to DOM elements
  Element(RespoElement<T>),
  Referenced(Rc<RespoNode<T>>),
}

impl<T> From<RespoNode<T>> for Cirru
where
  T: Debug + Clone,
{
  fn from(value: RespoNode<T>) -> Self {
    match value {
      RespoNode::Component(RespoComponent { name, tree, .. }) => {
        Cirru::List(vec![Cirru::Leaf("::Component".into()), Cirru::from(name.as_ref()), (*tree).into()])
      }
      RespoNode::Element(RespoElement { name, children, .. }) => {
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
    Self::Element(RespoElement {
      name: name.into(),
      attrs: HashMap::new(),
      event: HashMap::new(),
      style: RespoStyle::default(),
      children: Vec::new(),
    })
  }
  /// create a new component
  pub fn new_component(name: &str, tree: RespoNode<T>) -> Self {
    Self::Component(RespoComponent {
      name: name.into(),
      effects: Vec::new(),
      tree: Box::new(tree),
    })
  }
  /// wrap with a `Rc<T>` to enable memory reuse and skipping in diff
  pub fn rc(&self) -> Self {
    Self::Referenced(Rc::new(self.to_owned()))
  }
}

pub(crate) type StrDict = HashMap<Rc<str>, String>;

pub(crate) fn str_dict_to_cirrus_dict(dict: &StrDict) -> Cirru {
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

/// guide for actions to be dispatched
/// expecially for how you update states
pub trait RespoAction {
  /// to provide syntax sugar to dispatch.run_state
  fn build_states_action(cursor: &[Rc<str>], a: Option<RespoStateBranch>) -> Self
  where
    Self: Sized,
  {
    // val is a backup value from DynEq to Json Value
    let val = match &a {
      None => None,
      Some(v) => v.0.as_ref().backup(),
    };
    Self::states_action(RespoUpdateState {
      cursor: cursor.to_vec(),
      data: a,
      backup: val,
    })
  }

  /// a builder for states change
  fn states_action(a: RespoUpdateState) -> Self;
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
    (self.0)(T::build_states_action(cursor, Some(RespoStateBranch::new(a))))
  }
  /// reset state to empty
  pub fn run_empty_state(&self, cursor: &[Rc<str>]) -> Result<(), String> {
    (self.0)(T::build_states_action(cursor, None))
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
pub(crate) struct RespoEventMarkFn(Rc<dyn Fn(RespoEventMark) -> Result<(), String>>);

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
