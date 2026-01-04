use std::any::Any;
use std::fmt::{self, Debug};
use std::rc::Rc;

use crate::node::DispatchFn;

/// Type alias for a global event handler function.
pub type GlobalEventHandlerFn<T> = dyn Fn(&dyn GlobalEvent, &GlobalEventCtx<T>) -> Result<(), String> + 'static;

/// Trait implemented by events that can be broadcast via the global listeners pipeline.
///
/// Implementors gain runtime type information via `as_any`, which enables listeners
/// to downcast the incoming event to their concrete type.
pub trait GlobalEvent: Any + Send + Sync {
  /// Returns the event as `Any` to enable downcasting.
  fn as_any(&self) -> &dyn Any;

  /// Returns a descriptive event type name for debugging.
  fn name(&self) -> &'static str {
    std::any::type_name::<Self>()
  }
}

impl<T> GlobalEvent for T
where
  T: Any + Send + Sync,
{
  fn as_any(&self) -> &dyn Any {
    self
  }
}

/// Attempts to downcast a `GlobalEvent` reference to the requested concrete type.
pub fn downcast_event<E: GlobalEvent + 'static>(event: &dyn GlobalEvent) -> Option<&E> {
  event.as_any().downcast_ref::<E>()
}

/// Context passed into each global listener invocation.
///
/// - `dispatch` lets listeners trigger regular Respo actions or state updates.
/// - `component_path` contains the stack of component names leading to the current component.
pub struct GlobalEventCtx<'a, T>
where
  T: Debug + Clone,
{
  pub dispatch: &'a DispatchFn<T>,
  pub component_path: &'a [Rc<str>],
}

/// Wrapper around a global listener closure with pointer-based equality semantics.
pub struct GlobalEventHandler<T>
where
  T: Debug + Clone,
{
  inner: Rc<GlobalEventHandlerFn<T>>,
}

impl<T> Clone for GlobalEventHandler<T>
where
  T: Debug + Clone,
{
  fn clone(&self) -> Self {
    Self {
      inner: self.inner.to_owned(),
    }
  }
}

impl<T> GlobalEventHandler<T>
where
  T: Debug + Clone,
{
  pub fn new<F>(handler: F) -> Self
  where
    F: Fn(&dyn GlobalEvent, &GlobalEventCtx<T>) -> Result<(), String> + 'static,
  {
    Self { inner: Rc::new(handler) }
  }

  pub fn call(&self, event: &dyn GlobalEvent, ctx: &GlobalEventCtx<T>) -> Result<(), String> {
    (self.inner)(event, ctx)
  }
}

impl<T> PartialEq for GlobalEventHandler<T>
where
  T: Debug + Clone,
{
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.inner, &other.inner)
  }
}

impl<T> Eq for GlobalEventHandler<T> where T: Debug + Clone {}

impl<T> fmt::Debug for GlobalEventHandler<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("[GlobalEventHandler]")
  }
}

/// Convenience helper for building a `GlobalEventHandler` from a closure.
pub fn global_event_handler<T, F>(handler: F) -> GlobalEventHandler<T>
where
  T: Debug + Clone,
  F: Fn(&dyn GlobalEvent, &GlobalEventCtx<T>) -> Result<(), String> + 'static,
{
  GlobalEventHandler::new(handler)
}
