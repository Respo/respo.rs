use std::fmt::Debug;

use wasm_bindgen::prelude::Closure;
use web_sys::{Element, HtmlElement, HtmlInputElement, InputEvent, MouseEvent, Node};

use wasm_bindgen::JsCast;
use web_sys::console::warn_1;

use super::{track_delegated_event, DigitCoord, DomChange, RespoCoord, RespoEvent};

pub fn patch_tree<T>(mount_target: &Node, changes: &Vec<DomChange<T>>) -> Result<(), String>
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
    let target = find_coord_target(&mount_target.first_child().ok_or("to get first child")?, &coord)?;
    match op {
      DomChange::ModifyAttrs { set, unset, .. } => {
        let el = target.dyn_ref::<Element>().expect("load as element");
        for (k, v) in set {
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text(v);
          } else if k == "innerHTML" {
            el.set_inner_html(v);
          } else {
            el.set_attribute(k, v).map_err(|e| e.as_string().unwrap())?;
          }
        }
        for k in unset {
          if k == "innerText" {
            el.dyn_ref::<HtmlElement>().ok_or("to html element")?.set_inner_text("");
          } else if k == "innerHTML" {
            el.set_inner_html("");
          } else {
            el.remove_attribute(k).map_err(|e| e.as_string().unwrap())?;
          }
        }
      }
      DomChange::ModifyStyle { set, unset, .. } => {
        let style = mount_target.dyn_ref::<HtmlElement>().expect("into html element").style();
        for s in unset {
          style.remove_property(s).expect("remove style");
        }
        for (k, v) in set {
          style.set_property(k, v).expect("set style");
        }
      }
      DomChange::ModifyEvent {
        add, remove, respo_coord, ..
      } => {
        let el = target.dyn_ref::<Element>().expect("to element");
        for k in add.iter() {
          attach_event(el, k, respo_coord)?;
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
      DomChange::ReplaceElement { node, .. } => {}
      DomChange::ModifyChildren { operations, .. } => {
        // TODO
        warn_1(&"TODO children updates".into());
      }
    }
  }
  Ok(())
}

fn find_coord_target(mount_target: &Node, coord: &DigitCoord) -> Result<Node, String> {
  let mut target = mount_target.clone();
  for digit in &coord.0 {
    let child = target.child_nodes().item(*digit);
    if child.is_none() {
      return Err(format!("no child at index {}", digit));
    }
    target = child.ok_or_else(|| format!("does not find child at index: {}", digit))?;
  }
  Ok(target)
}

pub fn attach_event(element: &Element, key: &str, coord: &Vec<RespoCoord>) -> Result<(), String> {
  let coord = coord.to_owned();
  match key {
    "click" => {
      let handler = Closure::wrap(Box::new(move |e: MouseEvent| {
        track_delegated_event(
          &coord,
          "click",
          RespoEvent::Click {
            digit_coord: coord.to_owned().into(),
            respo_coord: coord.to_owned(),
            client_x: e.client_x() as f64,
            client_y: e.client_y() as f64,
          },
        );
      }) as Box<dyn FnMut(MouseEvent)>);
      element
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .set_onclick(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    "input" => {
      let handler = Closure::wrap(Box::new(move |e: InputEvent| {
        track_delegated_event(
          &coord,
          "input",
          RespoEvent::Input {
            digit_coord: coord.to_owned().into(),
            respo_coord: coord.to_owned(),
            value: e
              .target()
              .expect("to reach event target")
              .dyn_ref::<HtmlInputElement>()
              .unwrap()
              .value(),
          },
        );
      }) as Box<dyn FnMut(InputEvent)>);
      element
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .set_oninput(Some(handler.as_ref().unchecked_ref()));
      handler.forget();
    }
    _ => {
      warn_1(&format!("unhandled event: {}", key).into());
    }
  }
  Ok(())
}