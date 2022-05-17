use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

use crate::respo::primes::*;

use crate::respo::util::fst;

pub fn diff_tree<T>(
  new_tree: &RespoNode<T>,
  old_tree: &RespoNode<T>,
  digit_coord: DigitCoord,
  respo_coord: Vec<RespoCoord>,
  changes: &mut Vec<DomChange<T>>,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  match (new_tree, old_tree) {
    (RespoNode::Component(name, _, new_child), RespoNode::Component(name_old, _, old_child)) => {
      if name == name_old {
        let mut next_coord = respo_coord.clone();
        next_coord.push(RespoCoord::Comp(String::from(name)));
        diff_tree(new_child, old_child, digit_coord, next_coord, changes)?;
      } else {
        changes.push(DomChange::ReplaceElement {
          digit_coord,
          node: *old_child.to_owned(),
        });
      }
    }
    (RespoNode::Component(..), b) => {
      changes.push(DomChange::ReplaceElement {
        digit_coord,
        node: b.to_owned(),
      });
    }
    (_, b @ RespoNode::Component(..)) => {
      changes.push(DomChange::ReplaceElement {
        digit_coord,
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
          digit_coord,
          node: b.to_owned(),
        });
      } else {
        diff_attrs(attrs, old_attrs, &digit_coord, changes);
        diff_style(&style.0, &old_style.0, &digit_coord, changes);

        diff_event(event, old_event, &respo_coord, &digit_coord, changes);
        diff_children(children, old_children, &digit_coord, respo_coord, changes)?;
      }
    }
  }

  Ok(())
}

fn diff_attrs<T>(
  new_attrs: &HashMap<String, String>,
  old_attrs: &HashMap<String, String>,
  coord: &DigitCoord,
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
      digit_coord: coord.to_owned(),
      set: added,
      unset: removed,
    });
  }
}

fn diff_style<T>(
  new_style: &HashMap<String, String>,
  old_style: &HashMap<String, String>,
  coord: &DigitCoord,
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
      digit_coord: coord.to_owned(),
      set: added,
      unset: removed,
    });
  }
}

fn diff_event<T, U>(
  new_event: &HashMap<String, U>,
  old_event: &HashMap<String, U>,
  respo_coord: &Vec<RespoCoord>,
  digit_coord: &DigitCoord,
  changes: &mut Vec<DomChange<T>>,
) where
  T: Debug + Clone,
{
  let new_keys: HashSet<String> = new_event.keys().map(ToOwned::to_owned).collect();
  let old_keys: HashSet<String> = old_event.keys().map(ToOwned::to_owned).collect();

  if new_keys != old_keys {
    changes.push(DomChange::ModifyEvent {
      digit_coord: digit_coord.to_owned(),
      respo_coord: respo_coord.to_owned(),
      add: new_keys.difference(&old_keys).map(ToOwned::to_owned).collect(),
      remove: old_keys.difference(&new_keys).map(ToOwned::to_owned).collect(),
    });
  }
}

fn diff_children<T>(
  new_children: &[(RespoIndexKey, RespoNode<T>)],
  old_children: &[(RespoIndexKey, RespoNode<T>)],
  coord: &DigitCoord,
  respo_coord: Vec<RespoCoord>,
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
          changes.push(DomChange::ModifyChildren {
            digit_coord: coord.to_owned(),
            operations,
          });
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
        let mut next_coord = respo_coord.clone();
        next_coord.push(RespoCoord::Idx(cursor));
        diff_tree(&new_entry.1, &old_entry.1, coord.extend(cursor), next_coord, changes)?;
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
