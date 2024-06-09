pub(crate) mod effect;

use std::{fmt::Debug, rc::Rc};

use web_sys::Node;

use crate::{states_tree::DynEq, RespoEffect, RespoEffectArg, RespoEffectType, RespoNode};

/// internal abstraction for a component
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespoComponent<T>
where
  T: Debug + Clone,
{
  pub name: Rc<str>,
  pub effects: Vec<RespoEffect>,
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
  pub fn named(name: &str, tree: RespoNode<T>) -> Self {
    RespoComponent {
      name: Rc::from(name),
      effects: vec![],
      tree: Box::new(tree),
    }
  }
  pub fn to_node(self) -> RespoNode<T> {
    RespoNode::Component(self)
  }
  /// add an effect on component
  pub fn effect<U, V>(self, args: &[V], handler: U) -> Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
    V: Clone + DynEq + Debug + 'static,
  {
    let RespoComponent { name, effects, tree } = self;
    {
      // effects.push(RespoEffect::new(args.to_vec(), handler));
      let mut es = effects.to_owned();
      es.push(RespoEffect::new(args.to_vec(), handler));
      RespoComponent { name, effects: es, tree }
    }
  }
  /// add an empty args effect on component, which does not update
  pub fn stable_effect<U>(self, handler: U) -> Self
  where
    U: Fn(Vec<RespoEffectArg>, RespoEffectType, &Node) -> Result<(), String> + 'static,
  {
    let RespoComponent { name, effects, tree } = self;
    {
      let mut es = effects.to_owned();
      es.push(RespoEffect::new(vec![] as Vec<()>, handler));
      RespoComponent { name, effects: es, tree }
    }
  }
  /// add a list of effects on component
  pub fn effects<U>(self, more: U) -> Self
  where
    U: IntoIterator<Item = RespoEffect>,
  {
    let RespoComponent { name, effects, tree } = self;
    {
      let mut es = effects.to_owned();
      es.extend(more);
      RespoComponent { name, effects: es, tree }
    }
  }
}
