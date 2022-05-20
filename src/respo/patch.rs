use std::fmt::Debug;

use wasm_bindgen::prelude::Closure;
use web_sys::{console::log_1, Element, HtmlElement, HtmlInputElement, InputEvent, MouseEvent, Node};

use wasm_bindgen::JsCast;
use web_sys::console::warn_1;

use super::{
  build_dom_tree, load_coord_target_tree, ChildDomOp, DomChange, EventHandlerFn, RespoCoord, RespoEffectType, RespoEvent,
  RespoEventMark, RespoNode,
};

pub fn patch_tree<T>(
  tree: &RespoNode<T>,
  mount_target: &Node,
  changes: &Vec<DomChange<T>>,
  handle_event: EventHandlerFn,
) -> Result<(), String>
where
  T: Debug + Clone,
{
  // let el = mount_target.dyn_ref::<Element>().expect("to element");

  if mount_target.child_nodes().length() != 1 {
    return Err(format!(
      "expected a single node under mount target, got: {:?}",
      mount_target.child_nodes().length()
    ));
  }

  for op in changes {
    let coord = op.get_coord();
    let target = find_coord_dom_target(&mount_target.first_child().ok_or("to get first child")?, &coord)?;
    match op {
      DomChange::ModifyAttrs { set, unset, .. } => {
        let el = target.dyn_ref::<Element>().expect("load as element");
        for (k, v) in set {
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text(v);
          } else if k == "innerHTML" {
            el.set_inner_html(v);
          } else {
            el.set_attribute(k, v).expect("to set attribute");
          }
        }
        for k in unset {
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text("");
          } else if k == "innerHTML" {
            el.set_inner_html("");
          } else {
            el.remove_attribute(k).expect("to remove attribute");
          }
        }
      }
      DomChange::ModifyStyle { set, unset, .. } => {
        let style = target.dyn_ref::<HtmlElement>().expect("into html element").style();
        for s in unset {
          style.remove_property(s).expect("remove style");
        }
        for (k, v) in set {
          style.set_property(k, v).expect("set style");
        }
      }
      DomChange::ModifyEvent { add, remove, coord, .. } => {
        let el = target.dyn_ref::<Element>().expect("to element");
        for k in add.iter() {
          let handler = handle_event.clone();
          attach_event(el, k, coord, handler)?;
        }
        let el = el.dyn_ref::<HtmlElement>().expect("html element");
        for k in remove {
          match k.as_str() {
            "click" => {
              el.set_onclick(None);
            }
            "input" => {
              el.set_oninput(None);
            }
            _ => warn_1(&format!("TODO event {}", k).into()),
          }
        }
      }
      DomChange::ReplaceElement { node, .. } => {
        let parent = target.parent_element().expect("load parent");
        let handler = handle_event.clone();
        let new_element = build_dom_tree(node, &coord, handler).expect("build element");
        parent
          .dyn_ref::<Node>()
          .expect("to node")
          .insert_before(&target, Some(&new_element))
          .expect("element inserted");
        target.dyn_ref::<Element>().expect("get node").remove();
      }
      DomChange::ModifyChildren { operations, .. } => {
        for op in operations {
          let handler = handle_event.clone();
          match op {
            ChildDomOp::Append(node) => {
              let new_element = build_dom_tree(node, &coord, handler).expect("new element");
              target
                .dyn_ref::<Node>()
                .expect("to node")
                .append_child(&new_element)
                .expect("element appended");
            }
            ChildDomOp::RemoveAt(idx) => {
              let child = target
                .dyn_ref::<Element>()
                .expect("get node")
                .children()
                .item(*idx)
                .ok_or_else(|| format!("child not found at {}", &idx))?;
              target.remove_child(&child).expect("child removed");
            }
            ChildDomOp::InsertAfter(idx, node) => {
              let children = target.dyn_ref::<Element>().expect("get node").children();
              if idx >= &children.length() {
                return Err(format!("child not found at {}", &idx));
              } else {
                let handler = handle_event.clone();
                let new_element = build_dom_tree(node, &coord, handler).expect("new element");
                if idx == &children.length() {
                  let child = children.item(*idx + 1).ok_or_else(|| format!("child not found at {}", &idx))?;
                  target.insert_before(&new_element, Some(&child)).expect("element inserted");
                } else {
                  target.append_child(&new_element).expect("element appended");
                }
              }
            }
          }
        }
      }

      DomChange::Effect {
        coord,
        effect_type,
        skip_indexes,
      } => {
        let target_tree = load_coord_target_tree(tree, coord)?;
        if let RespoNode::Component(_, effects, _) = target_tree {
          for (idx, effect) in effects.iter().enumerate() {
            if !skip_indexes.contains(&(idx as u32)) {
              effect.run(effect_type.to_owned(), &target)?;
            }
          }
        } else {
          warn_1(&format!("expected component for effects, got: {:?}", target_tree).into());
        }
      }
    }
  }
  Ok(())
}

fn find_coord_dom_target(mount_target: &Node, coord: &[RespoCoord]) -> Result<Node, String> {
  let mut target = mount_target.clone();
  for digit in coord {
    if let &RespoCoord::Idx(idx) = digit {
      let child = target.child_nodes().item(idx);
      if child.is_none() {
        return Err(format!("no child at index {}", &idx));
      }
      target = child.ok_or_else(|| format!("does not find child at index: {}", &idx))?;
    }
  }
  Ok(target)
}

pub fn attach_event(element: &Element, key: &str, coord: &Vec<RespoCoord>, handle_event: EventHandlerFn) -> Result<(), String> {
  let coord = coord.to_owned();
  // log_1(&format!("attach event {}", key).into());
  match key {
    "click" => {
      let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
        handle_event
          .run(RespoEventMark {
            name: "click".to_owned(),
            coord: coord.to_owned(),
            event_info: RespoEvent::Click {
              client_x: e.client_x() as f64,
              client_y: e.client_y() as f64,
              original_event: e,
            },
          })
          .expect("handle click event");
      }) as Box<dyn FnMut(MouseEvent)>);
      element
        .dyn_ref::<HtmlElement>()
        .expect("convert to html element")
        .set_onclick(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "input" => {
      let handler = Closure::wrap(Box::new(move |e: InputEvent| {
        handle_event
          .run(RespoEventMark {
            coord: coord.to_owned(),
            name: "input".to_owned(),
            event_info: RespoEvent::Input {
              value: e
                .target()
                .expect("to reach event target")
                .dyn_ref::<HtmlInputElement>()
                .expect("to convert to html input element")
                .value(),
              original_event: e,
            },
          })
          .expect("handle input event");
      }) as Box<dyn FnMut(InputEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_oninput(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    _ => {
      warn_1(&format!("unhandled event: {}", key).into());
    }
  }
  Ok(())
}
