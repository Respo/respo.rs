use std::collections::HashSet;

use std::fmt::Debug;

use cirru_parser::Cirru;

use crate::{RespoEffectType, RespoIndexKey, RespoNode, StrDict};

use super::str_dict_to_cirrus_dict;

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
      DomChange::ReplaceElement { coord, .. } => coord.to_owned(),
      DomChange::ModifyChildren { coord, .. } => coord.to_owned(),
      DomChange::ModifyAttrs { coord, .. } => coord.to_owned(),
      DomChange::ModifyStyle { coord, .. } => coord.to_owned(),
      DomChange::ModifyEvent { coord, .. } => coord.to_owned(),
      DomChange::Effect { coord, .. } => coord.to_owned(),
    }
  }
  pub fn get_dom_path(&self) -> Vec<u32> {
    match self {
      DomChange::ReplaceElement { dom_path, .. } => dom_path.to_owned(),
      DomChange::ModifyChildren { dom_path, .. } => dom_path.to_owned(),
      DomChange::ModifyAttrs { dom_path, .. } => dom_path.to_owned(),
      DomChange::ModifyStyle { dom_path, .. } => dom_path.to_owned(),
      DomChange::ModifyEvent { dom_path, .. } => dom_path.to_owned(),
      DomChange::Effect { dom_path, .. } => dom_path.to_owned(),
    }
  }
}

impl<T> From<DomChange<T>> for Cirru
where
  T: Debug + Clone,
{
  fn from(change: DomChange<T>) -> Self {
    match change {
      DomChange::Effect {
        coord,
        dom_path,
        effect_type,
        skip_indexes,
      } => {
        let xs = vec![
          "::effect".into(),
          effect_type.into(),
          coord_path_to_cirru(coord),
          dom_path_to_cirru(&dom_path),
          skip_indexes.iter().map(|x| Cirru::from(x.to_string())).collect::<Vec<_>>().into(),
        ];
        Cirru::List(xs)
      }
      DomChange::ReplaceElement { coord, dom_path, node } => {
        let xs = vec![
          "::replace-element".into(),
          coord_path_to_cirru(coord),
          dom_path_to_cirru(&dom_path),
          node.into(),
        ];
        Cirru::List(xs)
      }
      DomChange::ModifyChildren {
        coord,
        dom_path,
        operations,
      } => {
        let mut xs = vec!["::modify-children".into(), coord_path_to_cirru(coord), dom_path_to_cirru(&dom_path)];
        let mut ys = vec!["::operations".into()];
        for op in operations {
          ys.push(op.into());
        }
        xs.push(Cirru::List(ys));
        Cirru::List(xs)
      }
      DomChange::ModifyAttrs {
        coord,
        dom_path,
        set,
        unset,
      } => {
        let xs = vec![
          "::modify-attrs".into(),
          coord_path_to_cirru(coord),
          dom_path_to_cirru(&dom_path),
          str_dict_to_cirrus_dict(&set),
          unset.iter().map(Cirru::from).collect::<Vec<_>>().into(),
        ];
        Cirru::List(xs)
      }
      DomChange::ModifyStyle {
        coord,
        dom_path,
        set,
        unset,
      } => {
        let xs = vec![
          "::modify-style".into(),
          coord_path_to_cirru(coord),
          dom_path_to_cirru(&dom_path),
          str_dict_to_cirrus_dict(&set),
          unset.iter().map(Cirru::from).collect::<Vec<_>>().into(),
        ];
        Cirru::List(xs)
      }
      DomChange::ModifyEvent {
        coord,
        dom_path,
        add,
        remove,
      } => {
        let xs = vec![
          "::modify-event".into(),
          coord_path_to_cirru(coord),
          dom_path_to_cirru(&dom_path),
          add.iter().map(Cirru::from).collect::<Vec<_>>().into(),
          remove.iter().map(Cirru::from).collect::<Vec<_>>().into(),
        ];
        Cirru::List(xs)
      }
    }
  }
}

pub fn changes_to_cirru<T>(change: &[DomChange<T>]) -> Cirru
where
  T: Debug + Clone,
{
  let mut xs = vec!["::changes".into()];
  for c in change {
    xs.push(c.to_owned().into());
  }
  Cirru::List(xs)
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

impl<T> From<ChildDomOp<T>> for Cirru
where
  T: Debug + Clone,
{
  fn from(op: ChildDomOp<T>) -> Self {
    match op {
      ChildDomOp::InsertAfter(index, key, node) => {
        let xs = vec!["::insert-after".into(), Cirru::from(index.to_string()), key.into(), node.into()];
        Cirru::List(xs)
      }
      ChildDomOp::RemoveAt(index) => {
        let xs = vec!["::remove-at".into(), Cirru::from(index.to_string())];
        Cirru::List(xs)
      }
      ChildDomOp::Append(key, node) => {
        let xs = vec!["::append".into(), key.into(), node.into()];
        Cirru::List(xs)
      }
      ChildDomOp::Prepend(key, node) => {
        let xs = vec!["::prepend".into(), key.into(), node.into()];
        Cirru::List(xs)
      }
      ChildDomOp::NestedEffect {
        nested_coord,
        nested_dom_path,
        effect_type,
        skip_indexes,
      } => {
        let xs = vec![
          "::effect".into(),
          effect_type.into(),
          coord_path_to_cirru(nested_coord),
          nested_dom_path
            .iter()
            .map(|x| Cirru::from(x.to_string()))
            .collect::<Vec<_>>()
            .into(),
          skip_indexes.iter().map(|x| Cirru::from(x.to_string())).collect::<Vec<_>>().into(),
        ];
        Cirru::List(xs)
      }
    }
  }
}

/// coordinate system on RespoNode, to lookup among elements and components
#[derive(Debug, Clone)]
pub enum RespoCoord {
  Key(RespoIndexKey),
  /// for indexing by component name, even though there's only one of that
  Comp(String),
}

impl From<RespoCoord> for Cirru {
  fn from(coord: RespoCoord) -> Self {
    match coord {
      RespoCoord::Key(key) => key.into(),
      RespoCoord::Comp(name) => vec![Cirru::from("::Comp"), Cirru::from(name)].into(),
    }
  }
}

fn coord_path_to_cirru(coord: Vec<RespoCoord>) -> Cirru {
  let mut xs = vec!["::coord".into()];
  for c in coord {
    xs.push(c.into());
  }
  Cirru::List(xs)
}

fn dom_path_to_cirru(dom_path: &[u32]) -> Cirru {
  let mut xs = vec!["::dom-path".into()];
  for c in dom_path {
    xs.push(Cirru::from(c.to_string()));
  }
  Cirru::List(xs)
}
