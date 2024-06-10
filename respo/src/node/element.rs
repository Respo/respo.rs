pub mod alias;
use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  rc::Rc,
};

use crate::{DispatchFn, RespoEvent, RespoIndexKey, RespoListenerFn, RespoNode, RespoStyle};

/// internal abstraction for an element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespoElement<T>
where
  T: Debug + Clone,
{
  /// tagName
  pub name: Rc<str>,
  pub attrs: HashMap<Rc<str>, String>,
  pub event: HashMap<Rc<str>, RespoListenerFn<T>>,
  /// inlines styles, partially typed.
  /// there's also a macro called `static_styles` for inserting CSS rules
  pub style: RespoStyle,
  /// each child as a key like a string, by default generated from index,
  /// they are used in diffing, so it's better to be distinct, although not required to be.
  pub children: Vec<(RespoIndexKey, RespoNode<T>)>,
}

impl From<RespoElement<()>> for RespoNode<()> {
  fn from(el: RespoElement<()>) -> Self {
    RespoNode::Element(el)
  }
}

impl<T> RespoElement<T>
where
  T: Debug + Clone,
{
  pub fn named(name: &str) -> Self {
    RespoElement {
      name: Rc::from(name),
      attrs: HashMap::new(),
      event: HashMap::new(),
      style: RespoStyle::default(),
      children: Vec::new(),
    }
  }

  pub fn to_node(self) -> RespoNode<T> {
    RespoNode::Element(self)
  }

  /// attach styles
  /// ```ignore
  /// element.style(RespoStyle::default().margin(10))
  /// ```
  pub fn style(self, more: RespoStyle) -> Self {
    let el = self;
    let mut style = el.style.to_owned();
    for (k, v) in more.0.into_iter() {
      style.0.push((k.to_owned(), v.to_owned()));
    }
    RespoElement { style, ..el }
  }
  /// imparative way of updating style
  /// ```ignore
  /// element.modify_style(|s| {
  ///   if data > 1 {
  ///     s.color(CssColor::Red);
  ///   }
  /// });
  /// ```
  pub fn modify_style<U>(self, builder: U) -> Self
  where
    U: Fn(&mut RespoStyle),
  {
    let el = self;
    let mut style = el.style.to_owned();
    builder(&mut style);
    RespoElement { style, ..el }
  }
  /// set an attribute on element
  pub fn attribute<U, V>(self, property: U, value: V) -> Self
  where
    U: Into<Rc<str>> + ToOwned,
    V: Display,
  {
    let el = self;
    let mut a = el.attrs.to_owned();
    a.insert(property.into(), value.to_string());
    RespoElement { attrs: a, ..el.to_owned() }
  }
  /// set an attribute on element, but using `None` indicates noting
  pub fn maybe_attribute<U, V>(self, property: U, value: Option<V>) -> Self
  where
    U: Into<Rc<str>> + ToOwned,
    V: Display,
  {
    if let Some(v) = value {
      let el = self;
      let mut a = el.attrs.to_owned();
      a.insert(property.into(), v.to_string());
      RespoElement { attrs: a, ..el }
    } else {
      self
    }
  }
  pub fn on_click<U>(self, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("click", handler)
  }
  pub fn on_input<U>(self, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("input", handler)
  }
  /// handle keydown event
  pub fn on_keydown<U>(self, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("keydown", handler)
  }
  /// handle focus event
  pub fn on_focus<U>(self, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("focus", handler)
  }
  /// handle change event
  pub fn on_change<U>(self, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    self.on_named_event("change", handler)
  }
  /// attach a listener by event name(only a small set of events are supported)
  pub fn on_named_event<U>(self, name: &str, handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    let el = self;
    let mut e = el.event.to_owned();
    e.insert(name.into(), RespoListenerFn::new(handler));
    RespoElement { event: e, ..el }
  }
  /// add children elements,
  /// index key are generated from index number
  pub fn children<U>(self, more: U) -> Self
  where
    U: IntoIterator<Item = RespoNode<T>>,
  {
    let el = self;
    let mut children = el.children.to_owned();
    for (idx, v) in more.into_iter().enumerate() {
      children.push((idx.into(), v.to_owned()));
    }
    RespoElement { children, ..el }
  }
  /// add children elements, with index keys specified
  pub fn children_indexed<U>(self, more: U) -> Self
  where
    U: IntoIterator<Item = (RespoIndexKey, RespoNode<T>)>,
  {
    let el = self;
    let mut children = el.children.to_owned();
    for (idx, v) in more {
      children.push((idx, v));
    }
    RespoElement { children, ..el }
  }

  /// add elements. if any component is involved, use `self.children([])` instead
  pub fn elements<U>(self, mode: U) -> Self
  where
    U: IntoIterator<Item = RespoElement<T>>,
  {
    let el = self;
    let mut children = el.children.to_owned();
    for (idx, v) in mode.into_iter().enumerate() {
      children.push((idx.into(), v.to_node()));
    }
    RespoElement { children, ..el }
  }

  /// attach a class name for adding styles
  pub fn class<U>(self, name: U) -> Self
  where
    U: Into<String>,
  {
    self.attribute("class", name.into())
  }
  /// attach an optional class name for adding styles
  pub fn maybe_class<U>(self, name: Option<U>) -> Self
  where
    U: Into<String>,
  {
    match name {
      Some(name) => self.attribute("class", name.into()),
      None => self,
    }
  }
  /// attach a class name, controlled by a boolean
  pub fn toggle_class<U>(self, name: U, on: bool) -> Self
  where
    U: Into<String>,
  {
    if on {
      self.attribute("class", name.into())
    } else {
      self
    }
  }
  /// attach a list of class names for adding styles
  pub fn class_list<U>(self, names: &[U]) -> Self
  where
    U: Into<String> + Clone,
  {
    let mut class_name: Vec<String> = vec![];
    for name in names {
      class_name.push((*name).to_owned().into());
    }
    self.attribute("class", class_name.join(" "))
  }
  /// writes `innerText`
  pub fn inner_text<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attribute("innerText", content.into())
  }
  /// writes `innerHTML`
  pub fn inner_html<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attribute("innerHTML", content.into())
  }
  /// writes `value`
  pub fn value<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attribute("value", content.into())
  }
}
