use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Node;

#[allow(dead_code)]
pub fn raf_loop(mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f_ = Rc::new(RefCell::new(None));
  let g = f_.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if let Err(e) = cb() {
      crate::log!("failed in raq loop: {}", e);
    }

    // Schedule ourself for another requestAnimationFrame callback.
    request_animation_frame(f_.borrow().as_ref().expect("call raq"));
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().expect("call raq"));
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("should register `requestAnimationFrame` OK");
}

fn window() -> web_sys::Window {
  web_sys::window().expect("no global `window` exists")
}

/// this API is used for development, prefer `req_loop` for fast response
#[allow(dead_code)]
pub fn raf_loop_slow(interval: i32, mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if let Err(e) = cb() {
      crate::log!("failed in slow loop: {}", e);
    }

    let f2 = f.clone();
    let h = Closure::wrap(Box::new(move || {
      request_animation_frame(f2.borrow().as_ref().expect("call raq"));
    }) as Box<dyn FnMut()>);
    web_sys::Window::set_timeout_with_callback_and_timeout_and_arguments_0(&window(), h.as_ref().unchecked_ref(), interval)
      .expect("call set timeout");
    h.forget(); // It is not good practice, just for simplification!

    // Schedule ourself for another requestAnimationFrame callback.
    // request_animation_frame(f.borrow().as_ref().expect("call raq"));
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().expect("call raq"));
}

// just get first of tuple
pub(crate) fn fst<T, U>(pair: &(T, U)) -> &T {
  &pair.0
}

/// a shorthand for get an Node with given pattern
pub fn query_select_node(pattern: &str) -> Result<Node, String> {
  let window = web_sys::window().expect("no global `window` exists");
  let document = window.document().expect("should have a document on window");
  let target = document.query_selector(pattern).expect("call selector").expect("find .app");

  if let Some(element) = target.dyn_ref::<Node>() {
    Ok(element.to_owned())
  } else {
    Err(format!("failed to find {}", pattern))
  }
}

/// wraps on top of `web_sys::console.log_1`, use it like:
/// ```ignore
/// util::log!("a is {}", a);
/// ```
#[macro_export]
macro_rules! log {
  ($($t:tt)*) => {{
    web_sys::console::log_1(&format!($($t)*).into());
  }};
}

pub use log;

pub fn cast_from_json<T>(data: &Value) -> T
where
  T: DeserializeOwned + Clone,
{
  serde_json::from_value(data.to_owned()).expect("should be json")
}

pub fn cast_into_json<T>(data: T) -> Value
where
  T: Serialize,
{
  serde_json::to_value(data).expect("should be json")
}
