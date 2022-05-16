mod alias;
mod primes;
mod util;

use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::{log_1, warn_1};
use web_sys::{Element, HtmlElement, Node};

pub use alias::*;
pub use primes::*;

lazy_static::lazy_static! {
  /// event queue that code in the loop will detect
  static ref EVENTS_QUEUE: RwLock<Vec<RespoEvent>> = RwLock::new(Vec::new());
}

fn load_user_events() -> Vec<RespoEvent> {
  let mut events = Vec::new();
  let mut queue = EVENTS_QUEUE.write().expect("to load events quwuw");
  for event in queue.drain(..) {
    events.push(event);
  }
  events
}

/// render elements
pub fn render_node(mount_target: &Element, tree: &RespoNode) -> Result<(), JsValue> {
  let element = build_dom_tree(tree, &[])?;

  mount_target.append_child(&element)?;

  log_1(&format!("render tree: {:?}", tree).into());

  util::raq_loop_slow(Box::new(move || {
    let events = load_user_events();

    // log_1(&"loop".into());

    if !events.is_empty() {
      log_1(&"todo event".into());
    }
  }));

  Ok(())
}

pub fn build_dom_tree(tree: &RespoNode, coord: &[RespoCoord]) -> Result<Node, JsValue> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  match tree {
    RespoNode::Component(name, _, child) => {
      let mut next_coord: Vec<RespoCoord> = coord.to_owned();
      next_coord.push(RespoCoord::Comp(name.clone()));
      build_dom_tree(child, &next_coord)
    }
    RespoNode::Element {
      name,
      attrs,
      style,
      event,
      children,
    } => {
      let element = document.create_element(name)?;
      for (key, value) in attrs {
        if key == "style" {
          warn_1(&"style is handled outside attrs".into());
        } else if key == "innerText" {
          element.dyn_ref::<HtmlElement>().unwrap().set_inner_text(value);
        } else if key == "innerHTML" {
          element.set_inner_html(value);
        } else {
          element.set_attribute(key, value)?;
        }
      }
      element.set_attribute("style", &style.to_string())?;
      for (idx, child) in children.iter().enumerate() {
        let mut next_coord = coord.to_owned();
        next_coord.push(RespoCoord::Idx(idx as u32));
        element.append_child(&build_dom_tree(child, &next_coord)?)?;
      }

      for (key, value) in event {
        match key.as_str() {
          "click" => {
            let c = coord.to_owned();
            let handler = Closure::wrap(Box::new(move || {
              track_delegated_event(&c, "click");
            }) as Box<dyn FnMut()>);
            element
              .dyn_ref::<HtmlElement>()
              .unwrap()
              .set_onclick(Some(handler.as_ref().unchecked_ref()));
            handler.forget();
          }
          "input" => {
            // TODO
          }
          _ => {
            warn_1(&format!("unhandled event: {}", key).into());
          }
        }
      }

      Ok(element.dyn_ref::<Node>().unwrap().clone())
    }
  }
}

pub fn track_delegated_event(coord: &[RespoCoord], name: &str) {
  let mut queue = EVENTS_QUEUE.write().expect("to track delegated event");
  queue.push(RespoEvent {
    name: name.to_owned(),
    coord: coord.to_owned(),
  });
}
