mod primes;
mod util;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::RwLock;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console::{log_1, log_2};

use primes::{RespoElementProps, RespoEvent};
use util::raq_loop;

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
pub fn render_element() {
  log_1(&"Respo TODOs".into());

  raq_loop(Box::new(move || {
    let events = load_user_events();
    if !events.is_empty() {
      log_1(&"todo event".into());
    }
  }));

  let f: Box<dyn Fn() -> Result<(), String>> = Box::new(|| {
    println!("TODO");
    Ok(())
  });

  let mut element_props = RespoElementProps {
    attrs: HashMap::new(),
    event: HashMap::from_iter([("click".to_string(), f)]),
    styles: HashMap::new(),
  };
  element_props.event.insert("next".to_string(), Box::new(|| Ok(())));

  (*element_props.event.get("next").unwrap())().unwrap();
}
