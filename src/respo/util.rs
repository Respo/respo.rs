use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

#[allow(dead_code)]
pub fn raq_loop(mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f_ = Rc::new(RefCell::new(None));
  let g = f_.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    cb().expect("called in raq loop");

    // Schedule ourself for another requestAnimationFrame callback.
    request_animation_frame(f_.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
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
pub fn raq_loop_slow(mut cb: Box<dyn FnMut() -> Result<(), String>>) {
  let f = Rc::new(RefCell::new(None));
  let g = f.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    cb().expect("called in raq loop");

    let f2 = f.clone();
    let h = Closure::wrap(Box::new(move || {
      request_animation_frame(f2.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>);
    web_sys::Window::set_timeout_with_callback_and_timeout_and_arguments_0(&window(), h.as_ref().unchecked_ref(), 180).unwrap();
    h.forget(); // It is not good practice, just for simplification!

    // Schedule ourself for another requestAnimationFrame callback.
    // request_animation_frame(f.borrow().as_ref().unwrap());
  }) as Box<dyn FnMut()>));

  request_animation_frame(g.borrow().as_ref().unwrap());
}

// just get first of tuple
pub fn fst<T, U>(pair: &(T, U)) -> &T {
  &pair.0
}
