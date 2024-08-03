use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::Node;

/// this one uses `requestAnimationFrame` for calling
#[allow(dead_code)]
pub fn raf_loop(mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f_ = Rc::new(RefCell::new(None));
  let g = f_.to_owned();

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

/// uses `requestAnimationFrame` for calling, but with a interval to reduce cost.
/// prefer `req_loop` if you want to be faster
#[allow(dead_code)]
pub fn raf_loop_slow(interval: i32, mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f = Rc::new(RefCell::new(None));
  let g = f.to_owned();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    if let Err(e) = cb() {
      crate::warn_log!(
        "Failure in slow loop, program has to stop since inconsistent DOM states. Details: {}",
        e
      );
    }

    let h = Closure::wrap(Box::new({
      let f = f.to_owned();
      move || {
        request_animation_frame(f.borrow().as_ref().expect("call raq"));
      }
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

/// wraps on top of `web_sys::console.log_1`.
///
/// use it like:
/// ```ignore
/// util::log!("a is {}", a);
/// ```
#[macro_export]
macro_rules! log {
  ($($t:tt)*) => {{
    web_sys::console::log_1(&format!($($t)*).into());
  }};
}

/// wraps on top of `web_sys::console.warn_1`.
///
/// use it like:
/// ```ignore
/// util::warn_log!("a is {}", a);
/// ```
#[macro_export]
macro_rules! warn_log {
  ($($t:tt)*) => {{
    web_sys::console::warn_1(&format!($($t)*).into());
  }};
}

/// wraps on top of `web_sys::console.error_1`.
///
/// use it like:
/// ```ignore
/// util::error_log!("a is {}", a);
/// ```
#[macro_export]
macro_rules! error_log {
  ($($t:tt)*) => {{
    web_sys::console::error_1(&format!($($t)*).into());
  }};
}

pub use error_log;
pub use log;
pub use warn_log;

/// display type of a variable, as a debug tool
#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
  // println!("{}", std::any::type_name::<T>())
  log!("{}", &std::any::type_name::<T>().to_string());
}
