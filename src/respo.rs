mod alias;
mod css;
mod diff;
mod patch;
mod primes;
mod states_tree;
pub mod util;

use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::{error_1, warn_1};
use web_sys::{HtmlElement, Node};

pub use alias::*;
pub use css::*;
pub use primes::*;
pub use states_tree::*;

use self::diff::{collect_effects_outside_in_as, diff_tree};
use self::patch::{attach_event, patch_tree};

lazy_static::lazy_static! {
  /// event queue that code in the loop will detect
  static ref NEED_TO_ERENDER: RwLock<bool> = RwLock::new(false);
}

/// check where need to trigger rerendering, also resets the status to false
fn drain_rerender_status() -> bool {
  let ret = { *NEED_TO_ERENDER.read().expect("to drain rerender status") };

  if ret {
    let mut need_to_erender = NEED_TO_ERENDER.write().expect("to drain rerender status");
    *need_to_erender = false;
  }
  ret
}

pub fn mark_need_rerender() {
  let ret = { *NEED_TO_ERENDER.read().expect("to drain rerender status") };

  if !ret {
    let mut need_to_erender = NEED_TO_ERENDER.write().expect("to drain rerender status");
    *need_to_erender = true;
  }
}

/// render elements
pub fn render_node<T>(
  mount_target: Node,
  mut renderer: Box<dyn FnMut() -> Result<RespoNode<T>, String>>,
  dispatch_action: DispatchFn<T>,
) -> Result<(), JsValue>
where
  T: 'static + Debug + Clone,
{
  let tree0: RespoNode<T> = renderer()?;
  let prev_tree = Rc::new(RefCell::new(tree0.clone()));

  let to_prev_tree = prev_tree.clone();
  let handle_event = EventHandlerFn::new(move |mark: RespoEventMark| -> Result<(), String> {
    match request_for_target_handler(&to_prev_tree.borrow(), &mark.name, &mark.coord) {
      Ok(handler) => match handler.run(mark.event_info, dispatch_action.clone()) {
        Ok(()) => {
          // util::log!("finished event: {} {:?}", mark.name, mark.coord);
          mark_need_rerender();
        }
        Err(e) => {
          error_1(&format!("event handler error: {:?}", e).into());
        }
      },
      Err(msg) => {
        error_1(&format!("event not handled: {}", msg).into());
      }
    }

    Ok(())
  });

  let handler = handle_event.clone();
  let element = build_dom_tree(&tree0, &[], handler)?;

  // collection mounted effects
  let mut mount_changes: Vec<DomChange<T>> = vec![];
  collect_effects_outside_in_as(&tree0, &[], &[], RespoEffectType::Mounted, &mut mount_changes)?;

  mount_target.append_child(&element)?;
  let handler = handle_event.clone();
  // util::log!("mounted changed: {:?}", mount_changes);
  patch_tree(&tree0, &prev_tree.borrow(), &mount_target, &mount_changes, handler)?;

  let to_prev_tree = prev_tree.clone();
  util::raf_loop_slow(Box::new(move || -> Result<(), String> {
    if drain_rerender_status() {
      let new_tree = renderer()?;
      let mut changes: Vec<DomChange<T>> = vec![];
      diff_tree(&new_tree, &to_prev_tree.borrow(), &Vec::new(), &Vec::new(), &mut changes)?;

      // util::log!("changes: {:?}", changes);

      let handler = handle_event.clone();
      patch_tree(&new_tree, &prev_tree.borrow(), &mount_target, &changes, handler)?;
      prev_tree.replace(new_tree);
    }

    Ok(())
  }));

  Ok(())
}

fn load_coord_target_tree<T>(tree: &RespoNode<T>, coord: &[RespoCoord]) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  // util::log!("looking for {:?}\n  {}", coord, &tree);
  if coord.is_empty() {
    Ok(tree.to_owned())
  } else {
    let branch = coord.first().ok_or("to get first branch of coord")?;
    match (tree, branch) {
      (RespoNode::Component(name, _, tree), RespoCoord::Comp(target_name)) => {
        if name == target_name {
          load_coord_target_tree(tree, &coord[1..])
        } else {
          Err(format!("expected component {} to be {}", &name, &target_name))
        }
      }
      (RespoNode::Element { children, .. }, RespoCoord::Key(idx)) => match children.iter().position(|(k, _)| idx == k) {
        Some(i) => {
          let child = &children.get(i).ok_or_else(|| format!("to get child {:?} {}", idx, i))?.1;
          load_coord_target_tree(child, &coord[1..])
        }
        None => Err(format!("no child at index key {:?}", idx)),
      },
      // match children.get(*idx as usize) {
      //   Some((_k, child)) => load_coord_target_tree(child, &coord[1..]),
      //   None => Err(format!("no child at index key {:?}", idx)),
      // },
      (RespoNode::Component(..), RespoCoord::Key(..)) => Err(String::from("expected element, found target being a component")),
      (RespoNode::Element { .. }, RespoCoord::Comp(..)) => {
        Err(format!("expected component at {:?}, found target being an element", coord))
      }
    }
  }
}

fn request_for_target_handler<T>(tree: &RespoNode<T>, event_name: &str, coord: &[RespoCoord]) -> Result<RespoEventHandler<T>, String>
where
  T: Debug + Clone,
{
  let target_node = load_coord_target_tree(tree, coord)?;

  match target_node {
    RespoNode::Component(name, ..) => Err(format!("expected element, found target being a component: {}", &name)),
    RespoNode::Element { name: tag_name, event, .. } => match event.get(event_name) {
      Some(v) => Ok((*v).to_owned()),
      None => Err(format!("no handler for event:{} on {} {:?}", &event_name, tag_name, event,)),
    },
  }
}

pub fn build_dom_tree<T>(tree: &RespoNode<T>, coord: &[RespoCoord], handle_event: EventHandlerFn) -> Result<Node, JsValue>
where
  T: Debug + Clone,
{
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  match tree {
    RespoNode::Component(name, _, child) => {
      let mut next_coord: Vec<RespoCoord> = coord.to_owned();
      next_coord.push(RespoCoord::Comp(name.clone()));
      build_dom_tree(child, &next_coord, handle_event)
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
          element.dyn_ref::<HtmlElement>().expect("into html element").set_inner_text(value);
        } else if key == "innerHTML" {
          element.set_inner_html(value);
        } else {
          element.set_attribute(key, value)?;
        }
      }
      element.set_attribute("style", &style.to_string())?;
      for (k, child) in children {
        let mut next_coord = coord.to_owned();
        next_coord.push(RespoCoord::Key(k.to_owned()));
        let handler = handle_event.clone();
        element.append_child(&build_dom_tree(child, &next_coord, handler)?)?;
      }

      // util::log!("create handler for element: {} {:?}", name, event);

      for key in event.keys() {
        let coord = coord.to_owned();
        let handler = handle_event.clone();
        attach_event(&element, key.as_str(), &coord, handler)?;
      }

      Ok(element.dyn_ref::<Node>().expect("converting to Node").clone())
    }
  }
}
