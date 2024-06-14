pub(crate) mod effect;

use std::{fmt::Debug, rc::Rc};

use effect::RespoEffectBox;

use crate::{RespoElement, RespoNode};

/// internal abstraction for a component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespoComponent<T>
where
  T: Debug + Clone,
{
  pub name: Rc<str>,
  pub effects: Vec<RespoEffectBox>,
  pub tree: Box<RespoNode<T>>,
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
    }
  }
  pub fn to_node(self) -> RespoNode<T> {
    RespoNode::Component(self)
  }

  /// add an effect on component
  pub fn effect(self, eff: RespoEffectBox) -> Self {
    let RespoComponent { name, mut effects, tree } = self;
    {
      effects.push(eff);
      RespoComponent { name, effects, tree }
    }
  }
  /// add an empty args effect on component, which does not update
  pub fn stable_effect(self, eff: RespoEffectBox) -> Self {
    let RespoComponent { name, mut effects, tree } = self;
    {
      effects.push(eff);
      RespoComponent { name, effects, tree }
    }
  }
  /// add a list of effects on component
  pub fn effects<U>(self, more: U) -> Self
  where
    U: IntoIterator<Item = RespoEffectBox>,
  {
    let RespoComponent { name, mut effects, tree } = self;
    {
      effects.extend(more);
      RespoComponent { name, effects, tree }
    }
  }
}
