use crate::app::util;
use crate::node::dom_change::RespoCoord;
use crate::node::{
  DispatchFn, DomChange, RespoComponent, RespoEffectType, RespoElement, RespoEventMark, RespoEventMarkFn, RespoListenerFn, RespoNode,
};
use crate::warn_log;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::console::{error_1, warn_1};
use web_sys::{HtmlElement, HtmlInputElement, HtmlLabelElement, HtmlTextAreaElement, Node};

use crate::app::diff::{collect_effects_outside_in_as, diff_tree};
use crate::app::patch::{attach_event, patch_tree};

lazy_static::lazy_static! {
  /// event queue that code in the loop will detect
  static ref NEED_TO_ERENDER: RwLock<bool> = RwLock::new(false);
}

/// check where need to trigger rerendering, also resets the status to false
fn drain_rerender_status() -> bool {
  let ret = { *NEED_TO_ERENDER.read().expect("to drain rerender status") };

  if ret {
    let mut need_to_rerender = NEED_TO_ERENDER.write().expect("to drain rerender status");
    *need_to_rerender = false;
  }
  ret
}

pub(crate) fn mark_need_rerender() {
  let ret = { *NEED_TO_ERENDER.read().expect("to drain rerender status") };

  if !ret {
    let mut need_to_erender = NEED_TO_ERENDER.write().expect("to drain rerender status");
    *need_to_erender = true;
  }
}

/// render elements
pub(crate) fn render_node<T, U>(
  mount_target: Node,
  // TODO it copies the whole store, need to optimize
  get_store: Box<dyn Fn() -> U>,
  mut renderer: Box<dyn FnMut() -> Result<RespoNode<T>, String>>,
  dispatch_action: DispatchFn<T>,
  interval: Option<i32>,
) -> Result<(), JsValue>
where
  T: 'static + Debug + Clone,
  U: Debug + Clone + 'static,
{
  let prev_store = RefCell::new(get_store());
  let tree0: RespoNode<T> = renderer()?;
  let prev_tree = Rc::new(RefCell::new(tree0.to_owned()));

  let to_prev_tree = prev_tree.to_owned();
  let handle_event = RespoEventMarkFn::new(move |mark: RespoEventMark| -> Result<(), String> {
    match request_for_target_handler(&to_prev_tree.borrow(), &mark.name, &mark.coord) {
      Ok(handler) => match handler.run(mark.event_info, dispatch_action.to_owned()) {
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

  let handler = handle_event.to_owned();
  let element = build_dom_tree(&tree0, &[], handler)?;

  // collection mounted effects
  let mut mount_changes: Vec<DomChange<T>> = vec![];
  collect_effects_outside_in_as(&tree0, &[], &[], RespoEffectType::Mounted, &mut mount_changes)?;

  mount_target.append_child(&element)?;
  let handler = handle_event.to_owned();
  // util::log!("mounted changed: {:?}", mount_changes);
  patch_tree(&tree0, &prev_tree.borrow(), &mount_target, &mount_changes, handler)?;

  let to_prev_tree = prev_tree.to_owned();
  match interval {
    Some(v) => {
      util::raf_loop_slow(
        v,
        Box::new(move || -> Result<(), String> {
          if drain_rerender_status() {
            let store = get_store();
            // if store == prev_store.borrow().to_owned() {
            //   // no need to update if store not changed
            //   return Ok(());
            // }

            prev_store.replace(store);

            let new_tree = renderer()?;
            let mut changes: Vec<DomChange<T>> = vec![];
            diff_tree(&new_tree, &to_prev_tree.borrow(), &Vec::new(), &Vec::new(), &mut changes)?;

            // use cirru_parser::CirruWriterOptions;
            // util::log!(
            //   "prev tree: {}",
            //   cirru_parser::format(&[to_prev_tree.borrow().to_owned().into()], CirruWriterOptions { use_inline: true }).unwrap()
            // );
            // use crate::dom_change::changes_to_cirru;
            // util::log!(
            //   "changes: {}",
            //   cirru_parser::format(&[changes_to_cirru(&changes)], CirruWriterOptions { use_inline: true }).unwrap()
            // );

            let handler = handle_event.to_owned();
            patch_tree(&new_tree, &prev_tree.borrow(), &mount_target, &changes, handler)?;
            prev_tree.replace(new_tree);
          }

          Ok(())
        }),
      );
    }
    None => {
      util::raf_loop(Box::new(move || -> Result<(), String> {
        if drain_rerender_status() {
          let new_tree = renderer()?;
          let mut changes: Vec<DomChange<T>> = vec![];
          diff_tree(&new_tree, &to_prev_tree.borrow(), &Vec::new(), &Vec::new(), &mut changes)?;

          let handler = handle_event.to_owned();
          patch_tree(&new_tree, &prev_tree.borrow(), &mount_target, &changes, handler)?;
          prev_tree.replace(new_tree);
        }

        Ok(())
      }));
    }
  }

  Ok(())
}

pub(crate) fn load_coord_target_tree<T>(tree: &RespoNode<T>, coord: &[RespoCoord]) -> Result<RespoNode<T>, String>
where
  T: Debug + Clone,
{
  // util::log!("looking for {:?}\n  {}", coord, &tree);
  if coord.is_empty() {
    match tree {
      RespoNode::Referenced(cell) => Ok((**cell).to_owned()),
      _ => Ok(tree.to_owned()),
    }
  } else {
    let branch = coord.first().ok_or("to get first branch of coord")?;
    match (tree, branch) {
      (RespoNode::Component(RespoComponent { name, tree, .. }), RespoCoord::Comp(target_name)) => {
        if name == target_name {
          load_coord_target_tree(tree, &coord[1..])
        } else {
          Err(format!(
            "Mismatch in expected component name: expected {}, found {}",
            &target_name, &name
          ))
        }
      }
      (RespoNode::Element(RespoElement { children, .. }), RespoCoord::Key(idx)) => match children.iter().position(|(k, _)| idx == k) {
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
      (RespoNode::Component(..), RespoCoord::Key(..)) => {
        Err(String::from("Type mismatch: expected a DOM element, but found a component"))
      }
      (RespoNode::Element { .. }, RespoCoord::Comp(..)) => {
        Err(format!("expected component at {:?}, found target being an element", coord))
      }
      (RespoNode::Referenced(cell), _) => load_coord_target_tree(cell, coord),
    }
  }
}

fn request_for_target_handler<T>(tree: &RespoNode<T>, event_name: &str, coord: &[RespoCoord]) -> Result<RespoListenerFn<T>, String>
where
  T: Debug + Clone,
{
  let target_node = load_coord_target_tree(tree, coord)?;

  match target_node {
    RespoNode::Component(RespoComponent { name, .. }) => Err(format!("expected element, found target being a component: {}", &name)),
    RespoNode::Element(RespoElement { name: tag_name, event, .. }) => match event.get(event_name) {
      Some(v) => Ok((*v).to_owned()),
      None => Err(format!(
        "No event handler found for event '{}' on element '{}', available events: {:?}",
        &event_name,
        tag_name,
        event.keys()
      )),
    },
    RespoNode::Referenced(cell) => request_for_target_handler(&cell, event_name, coord),
  }
}

/// creates a DOM tree from virtual DOM with proxied event handler attached
pub(crate) fn build_dom_tree<T>(tree: &RespoNode<T>, coord: &[RespoCoord], handle_event: RespoEventMarkFn) -> Result<Node, JsValue>
where
  T: Debug + Clone,
{
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");

  match tree {
    RespoNode::Component(RespoComponent { name, tree: child, .. }) => {
      let mut next_coord: Vec<RespoCoord> = coord.to_owned();
      next_coord.push(RespoCoord::Comp(name.to_owned()));
      build_dom_tree(child, &next_coord, handle_event)
    }
    RespoNode::Element(RespoElement {
      name,
      attributes: attrs,
      style,
      event,
      children,
    }) => {
      let element = document.create_element(name)?;
      let mut inner_set = false;
      for (key, value) in attrs {
        let key = key.as_ref();
        match key {
          "style" => warn_1(&"style is handled outside attrs".into()),
          "innerText" => {
            inner_set = true;
            element.dyn_ref::<HtmlElement>().expect("into html element").set_inner_text(value)
          }
          "innerHTML" => {
            inner_set = true;
            element.set_inner_html(value)
          }
          "htmlFor" => element
            .dyn_ref::<HtmlLabelElement>()
            .expect("into label element")
            .set_html_for(value),
          "value" if &**name == "textarea" => element
            .dyn_ref::<HtmlTextAreaElement>()
            .expect("into textarea element")
            .set_value(value),
          "value" if &**name == "input" => element.dyn_ref::<HtmlInputElement>().expect("into input element").set_value(value),
          _ => {
            element.set_attribute(key, value)?;
          }
        }
      }
      if !style.is_empty() {
        element.set_attribute("style", &style.to_string())?;
      }
      if inner_set && !children.is_empty() {
        warn_log!(
          "innerText or innerHTML is set, it's conflicted with children: {} {:?}",
          inner_set,
          children
        );
      }
      for (k, child) in children {
        let mut next_coord = coord.to_owned();
        next_coord.push(RespoCoord::Key(k.to_owned()));
        let handler = handle_event.to_owned();
        element.append_child(&build_dom_tree(child, &next_coord, handler)?)?;
      }

      // util::log!("create handler for element: {} {:?}", name, event);

      for key in event.keys() {
        let coord = coord.to_owned();
        let handler = handle_event.to_owned();
        attach_event(&element, key.as_ref(), &coord, handler)?;
      }

      Ok(element.dyn_ref::<Node>().expect("converting to Node").to_owned())
    }
    RespoNode::Referenced(cell) => build_dom_tree(cell, coord, handle_event),
  }
}
