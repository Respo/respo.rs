use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  rc::Rc,
};

use web_sys::Node;

use crate::{render_node, DispatchFn, RespoAction, RespoNode, RespoStore};

/// A template for a Respo app
pub trait RespoApp {
  /// a type of the store, with a place for states tree
  type Model: RespoStore + Debug + Clone + 'static;
  /// actions should include one for updating states tree
  type Action: Debug + Clone + RespoAction + 'static;

  /// simulating pure function updates to the model, but actually it's mutations
  fn dispatch(store: &mut RefMut<Self::Model>, action: Self::Action) -> Result<(), String>;

  /// bridge to mount target
  fn get_mount_target(&self) -> &Node;
  /// bridge to store
  fn get_store(&self) -> Rc<RefCell<Self::Model>>;
  /// default interval in milliseconds, by default 100ms,
  /// pass `None` to use raq directly, pass `Some(200)` to redice cost
  fn get_loop_delay() -> Option<i32> {
    Some(100)
  }

  /// DSL for building a view
  fn view(store: Ref<Self::Model>) -> Result<RespoNode<Self::Action>, String>;
  /// start a requestAnimationFrame loop for rendering updated store
  fn render_loop(&self) -> Result<(), String> {
    let mount_target = self.get_mount_target();
    let global_store = self.get_store();

    let store_to_action = global_store.clone();
    let store_to_action2 = global_store.clone();
    let dispatch_action = move |op: Self::Action| -> Result<(), String> {
      // util::log!("action {:?} store, {:?}", op, store_to_action.borrow());
      let mut store = store_to_action.borrow_mut();

      Self::dispatch(&mut store, op)?;
      // util::log!("store after action {:?}", store);
      Ok(())
    };

    render_node(
      mount_target.to_owned(),
      Box::new(move || store_to_action2.borrow().clone()),
      Box::new(move || -> Result<RespoNode<Self::Action>, String> {
        // util::log!("global store: {:?}", store);

        Self::view(global_store.borrow())
      }),
      DispatchFn::new(dispatch_action),
      Self::get_loop_delay(),
    )
    .expect("rendering node");

    Ok(())
  }
}
