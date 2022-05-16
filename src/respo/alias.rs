use std::{collections::HashMap, fmt::Debug};

use crate::respo::primes::RespoCssStyle;

use super::primes::{RespoEventHandler, RespoNode, StrDict};

pub fn div(attrs: StrDict, style: RespoCssStyle, event: HashMap<String, RespoEventHandler>, children: Vec<RespoNode>) -> RespoNode {
  RespoNode::Element {
    name: "div".to_owned(),
    attrs,
    event,
    style,
    children,
  }
}

pub fn span(attrs: StrDict, style: RespoCssStyle, event: HashMap<String, RespoEventHandler>, children: Vec<RespoNode>) -> RespoNode {
  RespoNode::Element {
    name: "span".to_owned(),
    attrs,
    event,
    style,
    children,
  }
}
