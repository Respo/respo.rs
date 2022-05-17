mod alias;
mod primes;
mod util;

use std::fmt::Debug;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::{error_1, log_1, warn_1};
use web_sys::{Element, HtmlElement, HtmlInputElement, Node};

pub use alias::*;
pub use primes::*;

lazy_static::lazy_static! {
  /// event queue that code in the loop will detect
  static ref EVENTS_QUEUE: RwLock<Vec<RespoEventMark>> = RwLock::new(Vec::new());
}

fn load_user_events() -> Vec<RespoEventMark> {
  let mut events = Vec::new();
  let mut queue = EVENTS_QUEUE.write().expect("to load events quwuw");
  for event in queue.drain(..) {
    events.push(event);
  }
  events
}

/// render elements
pub fn render_node<T>(
  mount_target: &Element,
  mut renderer: Box<dyn FnMut() -> Result<RespoNode<T>, String>>,
  dispatch_action: DispatchFn<T>,
) -> Result<(), JsValue>
where
  T: 'static + Debug + Clone,
{
  let tree: RespoNode<T> = renderer()?;
  let mut prev_tree = tree.clone();
  let element = build_dom_tree(&tree, &[])?;

  mount_target.append_child(&element)?;

  log_1(&format!("render tree: {:?}", tree).into());

  util::raq_loop_slow(Box::new(move || -> Result<(), String> {
    let event_marks = load_user_events();

    // log_1(&"loop".into());

    if !event_marks.is_empty() {
      for mark in event_marks {
        match request_for_target_handler(&tree, &mark.name, &mark.coord) {
          Ok(handler) => match (*handler.0)(RespoEvent::Click, dispatch_action.clone()) {
            Ok(()) => {
              log_1(&format!("finished event: {} {:?}", mark.name, mark.coord).into());
            }
            Err(e) => {
              error_1(&format!("event handler error: {:?}", e).into());
            }
          },
          Err(msg) => {
            error_1(&format!("event not handled: {}", msg).into());
          }
        }
      }
      let new_tree = renderer()?;
      let changes = tree_diff(&new_tree, &prev_tree);
      prev_tree = new_tree;
    }

    Ok(())
  }));

  Ok(())
}

fn request_for_target_handler<T>(tree: &RespoNode<T>, name: &str, coord: &[RespoCoord]) -> Result<RespoEventHandler<T>, String>
where
  T: Debug + Clone,
{
  if coord.is_empty() {
    match tree {
      RespoNode::Component(name, ..) => Err(format!("expected element, found target being a component: {}", &name)),
      RespoNode::Element { name: tag_name, event, .. } => match event.get(name) {
        Some(v) => Ok((*v).to_owned()),
        None => Err(format!("no handler for event {} on {}", &name, tag_name)),
      },
    }
  } else {
    let branch = coord.first().expect("to get first branch of coord");
    match (tree, branch) {
      (RespoNode::Component(name, _, tree), RespoCoord::Comp(target_name)) => {
        if name == target_name {
          request_for_target_handler(tree, name, &coord[1..])
        } else {
          Err(format!("expected component {} to be {}", &name, &target_name))
        }
      }
      (RespoNode::Element { children, .. }, RespoCoord::Idx(idx)) => match children.get(*idx as usize) {
        Some(child) => request_for_target_handler(child, name, &coord[1..]),
        None => Err(format!("no child at index {}", idx)),
      },
      (RespoNode::Component(..), RespoCoord::Idx(..)) => Err(String::from("expected element, found target being a component")),
      (RespoNode::Element { .. }, RespoCoord::Comp(..)) => Err(String::from("expected component, found target being an element")),
    }
  }
}

pub fn build_dom_tree<T>(tree: &RespoNode<T>, coord: &[RespoCoord]) -> Result<Node, JsValue>
where
  T: Debug + Clone,
{
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

      for key in event.keys() {
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
            let c = coord.to_owned();
            let handler = Closure::wrap(Box::new(move || {
              track_delegated_event(&c, "input");
            }) as Box<dyn FnMut()>);
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
      }

      Ok(element.dyn_ref::<Node>().unwrap().clone())
    }
  }
}

pub fn track_delegated_event(coord: &[RespoCoord], name: &str) {
  let mut queue = EVENTS_QUEUE.write().expect("to track delegated event");
  queue.push(RespoEventMark {
    name: name.to_owned(),
    coord: coord.to_owned(),
  });
}

pub fn tree_diff<T>(new_tree: &RespoNode<T>, old_tree: &RespoNode<T>) -> Vec<DomChange<T>>
where
  T: Debug + Clone,
{
  // TODO
  vec![]
}
