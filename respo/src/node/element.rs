pub mod alias;
use std::{
  collections::HashMap,
  fmt::{Debug, Display},
  rc::Rc,
};

use crate::{css::RespoStyle, DispatchFn, RespoEvent, RespoIndexKey, RespoListenerFn, RespoNode};

use super::css::respo_style;

/// internal abstraction for an element
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespoElement<T>
where
  T: Debug + Clone,
{
  /// tagName
  pub name: Rc<str>,
  pub attributes: HashMap<Rc<str>, String>,
  pub(crate) event: HashMap<Rc<str>, RespoListenerFn<T>>,
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
      attributes: HashMap::new(),
      event: HashMap::new(),
      style: respo_style(),
      children: Vec::new(),
    }
  }

  pub fn to_node(self) -> RespoNode<T> {
    RespoNode::Element(self)
  }

  /// attach styles
  /// ```ignore
  /// element.style(respo_style().margin(10))
  /// ```
  pub fn style(self, more: RespoStyle) -> Self {
    let mut style = self.style;
    for (k, v) in more.0.into_iter() {
      style.0.push((k.to_owned(), v.to_owned()));
    }
    RespoElement { style, ..self }
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
    let mut style = self.style;
    builder(&mut style);
    RespoElement { style, ..self }
  }
  /// set an attribute on element
  pub fn attr<U, V>(self, property: U, value: V) -> Self
  where
    U: Into<Rc<str>> + ToOwned,
    V: Display,
  {
    let mut attrs = self.attributes.to_owned();
    attrs.insert(property.into(), value.to_string());
    RespoElement {
      attributes: attrs,
      ..self.to_owned()
    }
  }
  /// set an attribute on element, but using `None` indicates noting
  pub fn maybe_attr<U, V>(self, property: U, value: Option<V>) -> Self
  where
    U: Into<Rc<str>> + ToOwned,
    V: Display,
  {
    if let Some(v) = value {
      let mut attrs = self.attributes.to_owned();
      attrs.insert(property.into(), v.to_string());
      RespoElement { attributes: attrs, ..self }
    } else {
      self
    }
  }
  /// set attributes from list of string pairs
  pub fn attrs<V>(self, list: &[(&str, V)]) -> Self
  where
    V: AsRef<str>,
  {
    let mut attrs = self.attributes.to_owned();
    for (k, v) in list {
      attrs.insert((*k).into(), v.as_ref().to_owned());
    }
    RespoElement { attributes: attrs, ..self }
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
    let mut children = self.children;
    for (idx, v) in more {
      children.push((idx, v));
    }
    RespoElement { children, ..self }
  }

  /// add elements. if any component is involved, use `self.children([])` instead
  pub fn elements<U>(self, mode: U) -> Self
  where
    U: IntoIterator<Item = RespoElement<T>>,
  {
    let mut children = self.children.to_owned();
    for (idx, v) in mode.into_iter().enumerate() {
      children.push((idx.into(), v.to_node()));
    }
    RespoElement { children, ..self }
  }

  /// attach a class name for adding styles
  pub fn class<U>(self, name: U) -> Self
  where
    U: Into<String>,
  {
    self.attr("class", name.into())
  }
  /// attach an optional class name for adding styles
  pub fn maybe_class<U>(self, name: Option<U>) -> Self
  where
    U: Into<String>,
  {
    match name {
      Some(name) => self.attr("class", name.into()),
      None => self,
    }
  }
  /// attach a class name, controlled by a boolean
  pub fn toggle_class<U>(self, name: U, on: bool) -> Self
  where
    U: Into<String>,
  {
    if on {
      self.attr("class", name.into())
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
    self.attr("class", class_name.join(" "))
  }
  /// writes `innerText`
  pub fn inner_text<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attr("innerText", content.into())
  }
  /// writes `innerHTML`
  pub fn inner_html<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attr("innerHTML", content.into())
  }
  /// writes `value`
  pub fn value<U>(self, content: U) -> Self
  where
    U: Into<String>,
  {
    self.attr("value", content.into())
  }
}
