use std::boxed::Box;
use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;
use std::{collections::HashMap, fmt::Debug};

use cirru_parser::{Cirru, CirruWriterOptions};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use web_sys::{InputEvent, KeyboardEvent, MouseEvent, Node};

use crate::{MaybeState, StatesTree};

use super::css::RespoStyle;

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
      RespoNode::Component(name, _eff, tree) => Cirru::List(vec![Cirru::Leaf(name.into()), (*tree).into()]),
      RespoNode::Element { name, children, .. } => Cirru::List(vec![
        Cirru::Leaf(name.into()),
        Cirru::List(
          children
            .iter()
            .map(|(k, child)| Cirru::List(vec![Cirru::Leaf(k.0.to_owned().into()), (*child).to_owned().into()]))
            .collect(),
        ),
      ]),
      RespoNode::Referenced(cell) => (*cell).to_owned().into(),
    }
  }
}

impl<T> Display for RespoNode<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match cirru_parser::format(&[self.to_owned().into()], CirruWriterOptions { use_inline: true }) {
      Ok(s) => write!(f, "{}", s),
      Err(e) => write!(f, "{}", e),
    }
  }
}

/// a key for referencing a child node, use a value that can be converted to string
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
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }

  pub fn insert_attr<U, V>(&mut self, property: U, value: V) -> &mut Self
  where
    U: Into<String> + ToOwned,
    V: Into<String> + ToOwned,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.insert_attr(property, value);
      }
      RespoNode::Element { ref mut attrs, .. } => {
        attrs.insert(property.into(), value.into());
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
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
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  pub fn on_click<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.on_click(handler);
      }
      RespoNode::Element { ref mut event, .. } => {
        event.insert("click".into(), RespoListenerFn::new(handler));
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  pub fn on_input<U>(&mut self, handler: U) -> &mut Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.on_input(handler);
      }
      RespoNode::Element { ref mut event, .. } => {
        event.insert("input".into(), RespoListenerFn::new(handler));
      }
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  pub fn add_event<U, V>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (V, RespoListenerFn<T>)>,
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
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  /// index key are generated from index number
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
      RespoNode::Referenced(_) => {
        unreachable!("should not be called on a referenced node");
      }
    }
    self
  }
  pub fn add_children_indexed<U>(&mut self, more: U) -> &mut Self
  where
    U: IntoIterator<Item = (RespoIndexKey, RespoNode<T>)>,
  {
    match self {
      RespoNode::Component(_, _, node) => {
        node.add_children_indexed(more);
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
    self.add_attrs([("class", name.into())])
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
    self.insert_attr("class", class_name.join(" "));
    self
  }
  /// writes `innerText`
  pub fn inner_text<U>(&mut self, content: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.insert_attr("innerText", content.into());
    self
  }
  /// writes `innerHTML`
  pub fn inner_html<U>(&mut self, content: U) -> &mut Self
  where
    U: Into<String>,
  {
    self.insert_attr("innerHTML", content.into());
    self
  }
  /// wrap with a `Rc<RefCell<T>>` to enable memory reuse and skipping in diff
  pub fn share_with_ref(&self) -> Self {
    Self::Referenced(Rc::new(self.clone()))
  }
}

pub(crate) type StrDict = HashMap<String, String>;

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

/// coordinate system on RespoNode, to lookup among elements and components
#[derive(Debug, Clone)]
pub enum RespoCoord {
  Key(RespoIndexKey),
  /// for indexing by component name, even though there's only one of that
  Comp(String),
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
  pub fn new<U, V>(args: Vec<&V>, handler: U) -> Self
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

/// DOM operations used for diff/patching
/// performance is not optimial since looking up the DOM via dom_path has repetitive operations,
/// might need to fix in future is overhead observed.
#[derive(Debug, Clone)]
pub enum DomChange<T>
where
  T: Debug + Clone,
{
  ReplaceElement {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
    node: RespoNode<T>,
  },
  ModifyChildren {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
    operations: Vec<ChildDomOp<T>>,
  },
  ModifyAttrs {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyStyle {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
    set: StrDict,
    unset: HashSet<String>,
  },
  ModifyEvent {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
    add: HashSet<String>,
    remove: HashSet<String>,
  },
  /// this is only part of effects.
  /// effects that collected while diffing children are nested inside
  Effect {
    coord: Vec<RespoCoord>,
    dom_path: Vec<u32>,
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
  pub fn get_dom_path(&self) -> Vec<u32> {
    match self {
      DomChange::ReplaceElement { dom_path, .. } => dom_path.clone(),
      DomChange::ModifyChildren { dom_path, .. } => dom_path.clone(),
      DomChange::ModifyAttrs { dom_path, .. } => dom_path.clone(),
      DomChange::ModifyStyle { dom_path, .. } => dom_path.clone(),
      DomChange::ModifyEvent { dom_path, .. } => dom_path.clone(),
      DomChange::Effect { dom_path, .. } => dom_path.clone(),
    }
  }
}

/// used in list diffing, this is still part of `DomChange`
#[derive(Debug, Clone)]
pub enum ChildDomOp<T>
where
  T: Debug + Clone,
{
  InsertAfter(u32, RespoIndexKey, RespoNode<T>),
  RemoveAt(u32),
  Append(RespoIndexKey, RespoNode<T>),
  Prepend(RespoIndexKey, RespoNode<T>),
  /// order is required in operating children elements, so put effect inside
  NestedEffect {
    nested_coord: Vec<RespoCoord>,
    nested_dom_path: Vec<u32>,
    effect_type: RespoEffectType,
    // when args not changed in update, that effects are not re-run
    skip_indexes: HashSet<u32>,
  },
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
