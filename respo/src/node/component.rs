pub(crate) mod effect;

use std::{fmt::Debug, rc::Rc};

use effect::RespoEffectBox;

use crate::node::global_event::GlobalEventHandler;
use crate::{RespoEffect, RespoElement, RespoNode};

/// internal abstraction for a component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespoComponent<T>
where
  T: Debug + Clone,
{
  pub name: Rc<str>,
  pub effects: Vec<RespoEffectBox>,
  pub tree: Box<RespoNode<T>>,
  /// Global event listeners registered on this component; order reflects registration sequence.
  pub listeners: Vec<GlobalEventHandler<T>>,
}

impl<T> From<RespoComponent<T>> for RespoNode<T>
where
  T: Debug + Clone,
{
  fn from(el: RespoComponent<T>) -> Self {
    RespoNode::Component(el)
  }
}

impl<T> RespoComponent<T>
where
  T: Debug + Clone,
{
  pub fn named(name: &str, tree: RespoElement<T>) -> Self {
    RespoComponent {
      name: Rc::from(name),
      effects: vec![],
      tree: Box::new(tree.to_node()),
      listeners: Vec::new(),
    }
  }
  pub fn to_node(self) -> RespoNode<T> {
    RespoNode::Component(self)
  }

  /// add an effect on component
  pub fn effect<S>(self, eff: S) -> Self
  where
    S: RespoEffect + 'static,
  {
    let RespoComponent {
      name,
      mut effects,
      tree,
      listeners,
    } = self;
    {
      effects.push(RespoEffectBox::new(eff));
      RespoComponent {
        name,
        effects,
        tree,
        listeners,
      }
    }
  }

  /// append a global event listener to this component.
  pub fn push_listener(mut self, handler: GlobalEventHandler<T>) -> Self {
    self.listeners.push(handler);
    self
  }

  /// clears all registered global listeners on this component.
  pub fn clear_listeners(&mut self) {
    self.listeners.clear();
  }
}
