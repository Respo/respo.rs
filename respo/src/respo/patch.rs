use std::cmp::Ordering;
use std::fmt::Debug;

use wasm_bindgen::prelude::Closure;
use web_sys::{
  Element, FocusEvent, HtmlElement, HtmlInputElement, HtmlLabelElement, HtmlTextAreaElement, InputEvent, KeyboardEvent, MouseEvent,
  Node,
};

use wasm_bindgen::JsCast;
use web_sys::console::warn_1;

use crate::{RespoComponent, RespoEffectType};

use super::{
  build_dom_tree, load_coord_target_tree, ChildDomOp, DomChange, RespoCoord, RespoEvent, RespoEventMark, RespoEventMarkFn, RespoNode,
};

pub fn patch_tree<T>(
  tree: &RespoNode<T>,
  old_tree: &RespoNode<T>,
  mount_target: &Node,
  changes: &[DomChange<T>],
  handle_event: RespoEventMarkFn,
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

  // handle BeforeUpdate before DOM changes
  for op in changes {
    if let DomChange::Effect {
      coord,
      effect_type,
      skip_indexes,
      ..
    } = op
    {
      if effect_type == &RespoEffectType::BeforeUpdate {
        let target = find_coord_dom_target(&mount_target.first_child().ok_or("mount position")?, op.get_dom_path())?;
        let target_tree = if effect_type == &RespoEffectType::BeforeUnmount {
          load_coord_target_tree(old_tree, coord)?
        } else {
          load_coord_target_tree(tree, coord)?
        };
        if let RespoNode::Component(RespoComponent(_, effects, _)) = target_tree {
          for (idx, effect) in effects.iter().enumerate() {
            if !skip_indexes.contains(&(idx as u32)) {
              effect.run(effect_type.to_owned(), &target)?;
            }
          }
        } else {
          crate::util::log!("expected component for effects, got: {}", target_tree);
        }
      }
    }
  }

  for op in changes {
    // crate::util::log!("op: {:?}", op);
    let target = find_coord_dom_target(&mount_target.first_child().ok_or("mount position")?, op.get_dom_path())?;
    match op {
      DomChange::ModifyAttrs { set, unset, .. } => {
        let el = target.dyn_ref::<Element>().expect("load as element");
        for (k, v) in set {
          let k = k.as_ref();
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text(v);
          } else if k == "innerHTML" {
            el.set_inner_html(v);
          } else if k == "htmlFor" {
            el.dyn_ref::<HtmlLabelElement>().ok_or("to label element")?.set_html_for(v);
          } else if k == "value" {
            match el.tag_name().as_str() {
              "INPUT" => {
                let input_el = el.dyn_ref::<HtmlInputElement>().expect("to input");
                let prev_value = input_el.value();
                if &prev_value != v {
                  input_el.set_value(v);
                }
              }
              "TEXTAREA" => {
                let textarea_el = el.dyn_ref::<HtmlTextAreaElement>().expect("to textarea");
                let prev_value = textarea_el.value();
                if &prev_value != v {
                  textarea_el.set_value(v);
                }
              }
              name => {
                return Err(format!("unsupported value for {}", name));
              }
            }
          } else {
            el.set_attribute(k, v).expect("to set attribute");
          }
        }
        for k in unset {
          let k = k.as_ref();
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text("");
          } else if k == "innerHTML" {
            el.set_inner_html("");
          } else if k == "value" {
            let input_el = el.dyn_ref::<HtmlInputElement>().expect("to input");
            let prev_value = input_el.value();
            if !prev_value.is_empty() {
              input_el.set_value("");
            }
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
          let handler = handle_event.to_owned();
          attach_event(el, k, coord, handler)?;
        }
        let el = el.dyn_ref::<HtmlElement>().expect("html element");
        for k in remove {
          match k.as_ref() {
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
      DomChange::ReplaceElement { node, coord, .. } => {
        let parent = target.parent_element().expect("load parent");
        let handler = handle_event.to_owned();
        let new_element = build_dom_tree(node, coord, handler).expect("build element");
        parent
          .dyn_ref::<Node>()
          .expect("to node")
          .insert_before(&new_element, Some(&target))
          .expect("element inserted");
        target.dyn_ref::<Element>().expect("get node").remove();
      }
      DomChange::ModifyChildren { operations, coord, .. } => {
        let base_tree = load_coord_target_tree(tree, coord)?;
        let old_base_tree = load_coord_target_tree(old_tree, coord)?;
        for op in operations {
          let handler = handle_event.to_owned();
          match op {
            ChildDomOp::Append(k, node) => {
              let mut next_coord = coord.to_owned();
              next_coord.push(RespoCoord::Key(k.to_owned()));
              let new_element = build_dom_tree(node, &next_coord, handler).expect("new element");
              target
                .dyn_ref::<Node>()
                .expect("to node")
                .append_child(&new_element)
                .expect("element appended");
            }
            ChildDomOp::Prepend(k, node) => {
              let mut next_coord = coord.to_owned();
              next_coord.push(RespoCoord::Key(k.to_owned()));
              let new_element = build_dom_tree(node, &next_coord, handler).expect("new element");
              if target.child_nodes().length() == 0 {
                target
                  .dyn_ref::<Node>()
                  .expect("to node")
                  .append_child(&new_element)
                  .expect("element appended");
              } else {
                let base = target.dyn_ref::<Node>().expect("to node").first_child().ok_or("to first child")?;
                target
                  .dyn_ref::<Node>()
                  .expect("to node")
                  .insert_before(&new_element, Some(&base))
                  .expect("element appended");
              }
            }
            ChildDomOp::RemoveAt(idx) => {
              let child = target
                .dyn_ref::<Element>()
                .expect("get node")
                .children()
                .item(*idx)
                .ok_or_else(|| format!("child to remove not found at {}", &idx))?;
              target.remove_child(&child).expect("child removed");
            }
            ChildDomOp::InsertAfter(idx, k, node) => {
              let children = target.dyn_ref::<Element>().expect("get node").children();
              if idx >= &children.length() {
                return Err(format!("child to insert not found at {}", &idx));
              } else {
                let handler = handle_event.to_owned();
                let mut next_coord = coord.to_owned();
                next_coord.push(RespoCoord::Key(k.to_owned()));
                let new_element = build_dom_tree(node, &next_coord, handler).expect("new element");
                match (idx + 1).cmp(&children.length()) {
                  Ordering::Less => {
                    let child = children.item(*idx + 1).ok_or_else(|| format!("child not found at {}", &idx))?;
                    target.insert_before(&new_element, Some(&child)).expect("element inserted");
                  }
                  Ordering::Equal => {
                    target.append_child(&new_element).expect("element appended");
                  }
                  Ordering::Greater => {
                    return Err(format!("out of bounds: {} of {} at coord {:?}", idx, &children.length(), coord));
                  }
                }
              }
            }
            ChildDomOp::NestedEffect {
              nested_coord,
              nested_dom_path: nesteed_dom_path,
              effect_type,
              skip_indexes,
            } => {
              let target_tree = if effect_type == &RespoEffectType::BeforeUnmount {
                load_coord_target_tree(&old_base_tree, nested_coord)?
              } else {
                load_coord_target_tree(&base_tree, nested_coord)?
              };
              let nested_el = find_coord_dom_target(&target, nesteed_dom_path)?;
              if let RespoNode::Component(RespoComponent(_, effects, _)) = target_tree {
                for (idx, effect) in effects.iter().enumerate() {
                  if !skip_indexes.contains(&(idx as u32)) {
                    effect.run(effect_type.to_owned(), &nested_el)?;
                  }
                }
              } else {
                crate::util::log!("expected component for effects, got: {}", target_tree);
              }
            }
          }
        }
      }

      DomChange::Effect {
        coord,
        effect_type,
        skip_indexes,
        ..
      } => {
        if effect_type == &RespoEffectType::BeforeUpdate {
          // should be handled before current pass
          continue;
        }
        let target_tree = if effect_type == &RespoEffectType::BeforeUnmount {
          load_coord_target_tree(old_tree, coord)?
        } else {
          load_coord_target_tree(tree, coord)?
        };
        if let RespoNode::Component(RespoComponent(_, effects, _)) = target_tree {
          for (idx, effect) in effects.iter().enumerate() {
            if !skip_indexes.contains(&(idx as u32)) {
              effect.run(effect_type.to_owned(), &target)?;
            }
          }
        } else {
          crate::util::log!("expected component for effects, got: {}", target_tree);
        }
      }
    }
  }
  Ok(())
}

fn find_coord_dom_target(mount_target: &Node, coord: &[u32]) -> Result<Node, String> {
  let mut target = mount_target.to_owned();
  for idx in coord {
    let child = target.child_nodes().item(idx.to_owned());
    if child.is_none() {
      return Err(format!("no child at index {}", &idx));
    }
    target = child.ok_or_else(|| format!("does not find child at index: {}", &idx))?;
  }
  Ok(target)
}

pub fn attach_event(element: &Element, key: &str, coord: &[RespoCoord], handle_event: RespoEventMarkFn) -> Result<(), String> {
  let coord = coord.to_owned();
  // crate::util::log!("attach event {}", key);
  match key {
    "click" => {
      let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
        let wrap_event = RespoEvent::Click {
          client_x: e.client_x() as f64,
          client_y: e.client_y() as f64,
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("click", &coord, wrap_event))
          .expect("handle click event");
      }) as Box<dyn FnMut(MouseEvent)>);
      element
        .dyn_ref::<HtmlElement>()
        .expect("convert to html element")
        .set_onclick(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }

    "dblclick" => {
      let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
        let wrap_event = RespoEvent::Click {
          client_x: e.client_x() as f64,
          client_y: e.client_y() as f64,
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("dblclick", &coord, wrap_event))
          .expect("handle dblclick event");
      }) as Box<dyn FnMut(MouseEvent)>);
      element
        .dyn_ref::<HtmlElement>()
        .expect("convert to html element")
        .set_ondblclick(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "input" => {
      let handler = Closure::wrap(Box::new(move |e: InputEvent| {
        let target = e.target().expect("get target");
        let el = target.dyn_ref::<Element>().unwrap();
        let value = match el.tag_name().as_str() {
          "INPUT" => el.dyn_ref::<HtmlInputElement>().expect("to convert to html input element").value(),
          "TEXTAREA" => el
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("to convert to html text area element")
            .value(),
          _ => {
            // TODO Error
            return;
          }
        };
        let wrap_event = RespoEvent::Input { value, original_event: e };
        handle_event
          .run(RespoEventMark::new("input", &coord, wrap_event))
          .expect("handle input event");
      }) as Box<dyn FnMut(InputEvent)>);
      match element.tag_name().as_str() {
        "INPUT" => {
          element
            .dyn_ref::<HtmlInputElement>()
            .expect("convert to html input element")
            .set_oninput(Some(handler.as_ref().unchecked_ref()));
        }
        "TEXTAREA" => {
          element
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("convert to html textarea element")
            .set_oninput(Some(handler.as_ref().unchecked_ref()));
        }
        _ => {
          return Err(format!("unsupported input event: {}", element.tag_name()));
        }
      }

      handler.forget();
    }
    "change" => {
      let handler = Closure::wrap(Box::new(move |e: InputEvent| {
        let wrap_event = RespoEvent::Input {
          value: e
            .target()
            .expect("to reach event target")
            .dyn_ref::<HtmlInputElement>()
            .expect("to convert to html input element")
            .value(),
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("change", &coord, wrap_event))
          .expect("handle change event");
      }) as Box<dyn FnMut(InputEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onchange(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "keydown" => {
      let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        // crate::util::log!("calling handler");
        let wrap_event = RespoEvent::Keyboard {
          key: e.key(),
          key_code: e.key_code(),
          shift_key: e.shift_key(),
          ctrl_key: e.ctrl_key(),
          alt_key: e.alt_key(),
          meta_key: e.meta_key(),
          repeat: e.repeat(),
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("keydown", &coord, wrap_event))
          .expect("handle keydown event");
      }) as Box<dyn FnMut(KeyboardEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onkeydown(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "keyup" => {
      let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let wrap_event = RespoEvent::Keyboard {
          key: e.key(),
          key_code: e.key_code(),
          shift_key: e.shift_key(),
          ctrl_key: e.ctrl_key(),
          alt_key: e.alt_key(),
          meta_key: e.meta_key(),
          repeat: e.repeat(),
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("keyup", &coord, wrap_event))
          .expect("handle keyup event");
      }) as Box<dyn FnMut(KeyboardEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onkeyup(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "keypress" => {
      let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        let wrap_event = RespoEvent::Keyboard {
          key: e.key(),
          key_code: e.key_code(),
          shift_key: e.shift_key(),
          ctrl_key: e.ctrl_key(),
          alt_key: e.alt_key(),
          meta_key: e.meta_key(),
          repeat: e.repeat(),
          original_event: e,
        };
        handle_event
          .run(RespoEventMark::new("keypress", &coord, wrap_event))
          .expect("handle keypress event");
      }) as Box<dyn FnMut(KeyboardEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onkeypress(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "focus" => {
      let handler = Closure::wrap(Box::new(move |e: FocusEvent| {
        handle_event
          .run(RespoEventMark::new("focus", &coord, RespoEvent::Focus(e)))
          .expect("handle focus event");
      }) as Box<dyn FnMut(FocusEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onfocus(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "blur" => {
      let handler = Closure::wrap(Box::new(move |e: FocusEvent| {
        handle_event
          .run(RespoEventMark::new("blur", &coord, RespoEvent::Focus(e)))
          .expect("handle blur event");
      }) as Box<dyn FnMut(FocusEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .expect("convert to html input element")
        .set_onblur(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    _ => {
      warn_1(&format!("unhandled event: {}", key).into());
    }
  }
  Ok(())
}
