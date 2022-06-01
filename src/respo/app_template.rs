use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  rc::Rc,
};

use web_sys::Node;

use crate::{render_node, ActionWithState, DispatchFn, MemoCache, RespoNode, StoreWithStates};

pub trait RespoApp {
  type Model: StoreWithStates + 'static;
  type Action: Debug + Clone + ActionWithState + 'static;

  /// simulating pure function updates to the model, but actually it's mutations
  fn dispatch(store: &mut RefMut<Self::Model>, action: Self::Action) -> Result<(), String>;

  fn get_mount_target(&self) -> &Node;
  fn get_store(&self) -> Rc<RefCell<Self::Model>>;
  fn get_memo_caches(&self) -> MemoCache<RespoNode<Self::Action>>;

  fn render_app(store: Ref<Self::Model>, memo_caches: MemoCache<RespoNode<Self::Action>>) -> Result<RespoNode<Self::Action>, String>;
  /// raq loop
  fn render_loop(&self) -> Result<(), String> {
    let mount_target = self.get_mount_target();
    let global_store = self.get_store();
    let memo_caches = self.get_memo_caches();

    let store_to_action = global_store.clone();
    let dispatch_action = move |op: Self::Action| -> Result<(), String> {
      // util::log!("action {:?} store, {:?}", op, store_to_action.borrow());
      let mut store = store_to_action.borrow_mut();

      Self::dispatch(&mut store, op)?;
      // util::log!("store after action {:?}", store);
      Ok(())
    };

    render_node(
      mount_target.to_owned(),
      Box::new(move || -> Result<RespoNode<Self::Action>, String> {
        // util::log!("global store: {:?}", store);

        Self::render_app(global_store.borrow(), memo_caches.clone())
      }),
      DispatchFn::new(dispatch_action),
    )
    .expect("rendering node");

    Ok(())
  }
}
