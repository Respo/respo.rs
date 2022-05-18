use std::{collections::HashMap, fmt::Debug, vec};

use crate::respo::css::RespoStyle;

use super::primes::{RespoEventHandler, RespoNode, StrDict};

#[allow(dead_code)]
pub fn div<T>(
  attrs: StrDict,
  style: RespoStyle,
  event: HashMap<String, RespoEventHandler<T>>,
  children: Vec<RespoNode<T>>,
) -> RespoNode<T>
where
  T: Debug + Clone,
{
  RespoNode::Element {
    name: "div".to_owned(),
    attrs,
    event,
    style,
    children: children
      .iter()
      .enumerate()
      .map(|(i, c)| (i.to_string().into(), c.clone()))
      .collect(),
  }
}

#[allow(dead_code)]
pub fn span<T>(
  attrs: StrDict,
  style: RespoStyle,
  event: HashMap<String, RespoEventHandler<T>>,
  children: Vec<RespoNode<T>>,
) -> RespoNode<T>
where
  T: Debug + Clone,
{
  RespoNode::Element {
    name: "span".to_owned(),
    attrs,
    event,
    style,
    children: children
      .iter()
      .enumerate()
      .map(|(i, c)| (i.to_string().into(), c.clone()))
      .collect(),
  }
}

#[allow(dead_code)]
pub fn span0<T>() -> RespoNode<T>
where
  T: Debug + Clone,
{
  RespoNode::Element {
    name: "span".to_owned(),
    attrs: HashMap::new(),
    event: HashMap::new(),
    style: RespoStyle::default(),
    children: vec![],
  }
}

#[allow(dead_code)]
pub fn div0<T>() -> RespoNode<T>
where
  T: Debug + Clone,
{
  RespoNode::Element {
    name: "span".to_owned(),
    attrs: HashMap::new(),
    event: HashMap::new(),
    style: RespoStyle::default(),
    children: vec![],
  }
}
