use std::{fmt::Debug, rc::Rc};

use web_sys::{FocusEvent, InputEvent, KeyboardEvent, MouseEvent};

use crate::node::{DispatchFn, RespoCoord};

/// (internal) struct to store event handler function on the tree
#[derive(Clone)]
pub struct RespoListenerFn<T>(Rc<dyn Fn(RespoEvent, DispatchFn<T>) -> Result<(), String>>)
where
  T: Debug + Clone;

impl<T> PartialEq for RespoListenerFn<T>
where
  T: Debug + Clone,
{
  /// returns true since informations are erased when attaching to the DOM
  fn eq(&self, _: &Self) -> bool {
    true
  }
}

impl<T> Eq for RespoListenerFn<T> where T: Debug + Clone {}

impl<T> Debug for RespoListenerFn<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RespoEventHandler(...)")
  }
}

impl<T> RespoListenerFn<T>
where
  T: Debug + Clone,
{
  pub fn new<U>(handler: U) -> Self
  where
    U: Fn(RespoEvent, DispatchFn<T>) -> Result<(), String> + 'static,
  {
    Self(Rc::new(handler))
  }
  pub fn run(&self, event: RespoEvent, dispatch: DispatchFn<T>) -> Result<(), String> {
    (self.0)(event, dispatch)
  }
}

/// marks on virtual DOM to declare that there's an event
/// event handler is HIDDEN from this mark.
#[derive(Debug, Clone)]
pub struct RespoEventMark {
  /// location of element in the tree
  pub coord: Vec<RespoCoord>,
  /// TODO event type
  pub name: String,
  /// partial copy of DOM events, that shares across threads,
  /// but being async, methods like `.prevent_default()` and `.stop_propagation()` will not work
  pub event_info: RespoEvent,
}

impl RespoEventMark {
  pub fn new(name: &str, coord: &[RespoCoord], event: RespoEvent) -> Self {
    Self {
      name: name.to_owned(),
      coord: coord.to_vec(),
      event_info: event,
    }
  }
}

/// event wraps on top of DOM events
#[derive(Debug, Clone)]
pub enum RespoEvent {
  Click {
    client_x: f64,
    client_y: f64,
    original_event: MouseEvent,
  },
  Keyboard {
    key: String,
    key_code: u32,
    shift_key: bool,
    ctrl_key: bool,
    alt_key: bool,
    meta_key: bool,
    repeat: bool,
    original_event: KeyboardEvent,
  },
  Input {
    value: String,
    original_event: InputEvent,
  },
  Focus(FocusEvent),
  Blur(FocusEvent),
}
