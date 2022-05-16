use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;

pub fn raq_loop(mut cb: Box<dyn FnMut()>) {
  let f_ = Rc::new(RefCell::new(None));
  let g = f_.clone();

  *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    cb();

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
