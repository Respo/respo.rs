use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::respo::primes::*;

use crate::respo::util::fst;

pub fn diff_tree<T>(
  new_tree: &RespoNode<T>,
  old_tree: &RespoNode<T>,
  coord: Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  match (new_tree, old_tree) {
    (RespoNode::Component(name, effects, new_child), RespoNode::Component(name_old, old_effects, old_child)) => {
      if name == name_old {
        let mut next_coord = coord.clone();
        next_coord.push(RespoCoord::Comp(String::from(name)));
        diff_tree(new_child, old_child, next_coord, changes)?;
        let mut skipped = HashSet::new();
        for (idx, effect) in effects.iter().enumerate() {
          if let Some(old_effect) = old_effects.get(idx) {
            if effect.args == old_effect.args {
              skipped.insert(idx as u32);
            }
          }
        }
        if skipped.len() < effects.len() {
          changes.push(DomChange::Effect {
            coord: coord.clone(),
            effect_type: RespoEffectType::BeforeUpdate,
            skip_indexes: skipped.to_owned(),
          });
          changes.push(DomChange::Effect {
            coord: coord.clone(),
            effect_type: RespoEffectType::Updated,
            skip_indexes: skipped.to_owned(),
          });
        }
      } else {
        collect_effects_outside_in_as(old_tree, coord.to_owned(), RespoEffectType::BeforeUnmount, changes)?;
        changes.push(DomChange::ReplaceElement {
          coord: coord.to_owned(),
          node: *old_child.to_owned(),
        });
        collect_effects_outside_in_as(new_tree, coord, RespoEffectType::Mounted, changes)?;
      }
    }
    (RespoNode::Component(..), b) => {
      changes.push(DomChange::ReplaceElement {
        coord: coord.to_owned(),
        node: b.to_owned(),
      });
      collect_effects_outside_in_as(new_tree, coord.to_owned(), RespoEffectType::Mounted, changes)?;
    }
    (_, b @ RespoNode::Component(..)) => {
      collect_effects_outside_in_as(old_tree, coord.to_owned(), RespoEffectType::BeforeUnmount, changes)?;
      changes.push(DomChange::ReplaceElement {
        coord: coord.to_owned(),
        node: b.to_owned(),
      });
    }
    (
      RespoNode::Element {
        name,
        attrs,
        style,
        event,
        children,
      },
      b @ RespoNode::Element {
        name: old_name,
        attrs: old_attrs,
        style: old_style,
        event: old_event,
        children: old_children,
      },
    ) => {
      if name != old_name {
        changes.push(DomChange::ReplaceElement {
          coord: coord.to_owned(),
          node: b.to_owned(),
        });
      } else {
        diff_attrs(attrs, old_attrs, &coord, changes);
        diff_style(&style.0, &old_style.0, &coord, changes);

        diff_event(event, old_event, &coord, changes);
        diff_children(children, old_children, coord, changes)?;
      }
    }
  }

  Ok(())
}

fn diff_attrs<T>(
  new_attrs: &HashMap<String, String>,
  old_attrs: &HashMap<String, String>,
  coord: &Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) where
  T: Debug + Clone,
{
  let mut added: StrDict = HashMap::new();
  let mut removed: HashSet<String> = HashSet::new();
  for (key, value) in new_attrs {
    if old_attrs.contains_key(key) {
      if &old_attrs[key] != value {
        added.insert(key.to_owned(), value.to_owned());
      }
    } else {
      added.insert(key.to_owned(), value.to_owned());
    }
  }

  for key in old_attrs.keys() {
    if !new_attrs.contains_key(key) {
      removed.insert(key.to_owned());
    }
  }

  if !added.is_empty() || !removed.is_empty() {
    changes.push(DomChange::ModifyAttrs {
      coord: coord.to_owned(),
      set: added,
      unset: removed,
    });
  }
}

fn diff_style<T>(
  new_style: &HashMap<String, String>,
  old_style: &HashMap<String, String>,
  coord: &Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) where
  T: Debug + Clone,
{
  let mut added: StrDict = HashMap::new();
  let mut removed: HashSet<String> = HashSet::new();
  for (key, value) in new_style {
    if old_style.contains_key(key) {
      if &old_style[key] != value {
        added.insert(key.to_owned(), value.to_owned());
      }
    } else {
      added.insert(key.to_owned(), value.to_owned());
    }
  }

  for key in old_style.keys() {
    if !new_style.contains_key(key) {
      removed.insert(key.to_owned());
    }
  }

  if !added.is_empty() || !removed.is_empty() {
    changes.push(DomChange::ModifyStyle {
      coord: coord.to_owned(),
      set: added,
      unset: removed,
    });
  }
}

fn diff_event<T, U>(
  new_event: &HashMap<String, U>,
  old_event: &HashMap<String, U>,
  coord: &Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) where
  T: Debug + Clone,
{
  let new_keys: HashSet<String> = new_event.keys().map(ToOwned::to_owned).collect();
  let old_keys: HashSet<String> = old_event.keys().map(ToOwned::to_owned).collect();

  if new_keys != old_keys {
    changes.push(DomChange::ModifyEvent {
      coord: coord.to_owned(),
      add: new_keys.difference(&old_keys).map(ToOwned::to_owned).collect(),
      remove: old_keys.difference(&new_keys).map(ToOwned::to_owned).collect(),
    });
  }
}

fn diff_children<T>(
  new_children: &[(RespoIndexKey, RespoNode<T>)],
  old_children: &[(RespoIndexKey, RespoNode<T>)],
  coord: Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  let mut cursor: u32 = 0;
  let mut operations: Vec<ChildDomOp<T>> = Vec::new();
  let mut new_tracking_pointer: usize = 0;
  let mut old_tracking_pointer: usize = 0;

  loop {
    if new_tracking_pointer >= new_children.len() {
      if old_tracking_pointer >= old_children.len() {
        if !operations.is_empty() {
          changes.push(DomChange::ModifyChildren { coord, operations });
        }

        return Ok(());
      } else {
        operations.push(ChildDomOp::RemoveAt(cursor));
        old_tracking_pointer += 1;
      }
    } else if old_tracking_pointer >= old_children.len() {
      operations.push(ChildDomOp::Append(new_children[new_tracking_pointer].1.to_owned()));
      new_tracking_pointer += 1;
    } else {
      let new_entry = &new_children[new_tracking_pointer];
      let old_entry = &old_children[old_tracking_pointer];
      if new_entry.0 == old_entry.0 {
        let mut next_coord = coord.clone();
        next_coord.push(RespoCoord::Idx(cursor));
        diff_tree(&new_entry.1, &old_entry.1, next_coord, changes)?;
        cursor += 1;
        new_tracking_pointer += 1;
        old_tracking_pointer += 1;
      } else if Some(&new_entry.0) == old_children.get(old_tracking_pointer + 1).map(fst)
        || Some(&new_entry.0) == old_children.get(old_tracking_pointer + 2).map(fst)
        || Some(&new_entry.0) == old_children.get(old_tracking_pointer + 3).map(fst)
      {
        // look ahead for 3 entries, if still not found, regards this as a remove
        operations.push(ChildDomOp::RemoveAt(cursor));
        old_tracking_pointer += 1;
      } else if Some(&old_entry.0) == new_children.get(new_tracking_pointer + 1).map(fst)
        || Some(&old_entry.0) == new_children.get(new_tracking_pointer + 2).map(fst)
        || Some(&old_entry.0) == new_children.get(new_tracking_pointer + 3).map(fst)
      {
        operations.push(ChildDomOp::Append(new_entry.1.to_owned()));
        cursor += 1;
        new_tracking_pointer += 1;
      } else {
        operations.push(ChildDomOp::RemoveAt(cursor));
        operations.push(ChildDomOp::InsertAfter(cursor, new_entry.1.to_owned()));

        cursor += 1;
        new_tracking_pointer += 1;
        old_tracking_pointer += 1;
      }
    }
  }
}

// effects at parent are collected first
pub fn collect_effects_outside_in_as<T>(
  tree: &RespoNode<T>,
  coord: Vec<RespoCoord>,
  effect_type: RespoEffectType,
  changes: &mut Vec<DomChange<T>>,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  match tree {
    RespoNode::Component(name, _, tree) => {
      changes.push(DomChange::Effect {
        coord: coord.to_owned(),
        effect_type,
        skip_indexes: HashSet::new(),
      });
      let mut next_coord = coord;
      next_coord.push(RespoCoord::Comp(name.to_owned()));
      collect_effects_outside_in_as(tree, next_coord, effect_type, changes)?;
      Ok(())
    }
    RespoNode::Element { children, .. } => {
      for (idx, (_, child)) in children.iter().enumerate() {
        let mut next_coord = coord.clone();
        next_coord.push(RespoCoord::Idx(idx as u32));
        collect_effects_outside_in_as(child, next_coord, effect_type, changes)?;
      }
      Ok(())
    }
  }
}

// effects deeper inside children are collected first
pub fn collect_effects_inside_out_as<T>(
  tree: &RespoNode<T>,
  coord: Vec<RespoCoord>,
  effect_type: RespoEffectType,
  changes: &mut Vec<DomChange<T>>,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  match tree {
    RespoNode::Component(name, _, tree) => {
      let mut next_coord = coord.to_owned();
      next_coord.push(RespoCoord::Comp(name.to_owned()));
      collect_effects_outside_in_as(tree, next_coord, effect_type, changes)?;
      changes.push(DomChange::Effect {
        coord,
        effect_type,
        skip_indexes: HashSet::new(),
      });
      Ok(())
    }
    RespoNode::Element { children, .. } => {
      for (idx, (_, child)) in children.iter().enumerate() {
        let mut next_coord = coord.clone();
        next_coord.push(RespoCoord::Idx(idx as u32));
        collect_effects_outside_in_as(child, next_coord, effect_type, changes)?;
      }
      Ok(())
    }
  }
}
