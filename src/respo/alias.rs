use std::{collections::HashMap, fmt::Debug};

use crate::respo::primes::RespoCssStyle;

use super::primes::{RespoEventHandler, RespoNode, StrDict};

pub fn div<T>(
  attrs: StrDict,
  style: RespoCssStyle,
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
    children,
  }
}

pub fn span<T>(
  attrs: StrDict,
  style: RespoCssStyle,
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
    children,
  }
}
