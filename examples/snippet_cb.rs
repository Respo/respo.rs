use std::{cell::RefCell, rc::Rc};

struct A {
  callback: Box<dyn FnMut() -> f32>,
}

pub fn main() {
  let b = Rc::new(RefCell::new(1.0));
  let mut a = A {
    callback: Box::new(move || {
      let mut bi = (*b).borrow_mut();
      let ret = *bi;
      *bi += 1.;
      ret
    }),
  };

  println!("{}", (a.callback)());

  println!("{}", (a.callback)());

  println!("{}", (a.callback)());

  println!("{}", (a.callback)());

  println!("{}", (a.callback)());
}
