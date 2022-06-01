use std::{
  cell::{Ref, RefCell, RefMut},
  fmt::Debug,
  rc::Rc,
};

use web_sys::Node;

use crate::{init_memo_cache, render_node, ActionWithState, DispatchFn, MemoCache, RespoNode, StoreWithStates};

pub trait RespoApp {
  type Model: StoreWithStates + 'static;
  type Action: Debug + Clone + ActionWithState + 'static;
  fn initial_model() -> Self::Model;
  /// simulating pure function updates to the model, but actually it's mutations
  fn dispatch(store: &mut RefMut<Self::Model>, action: Self::Action) -> Result<(), String>;

  fn get_mount_target(&self) -> &Node;

  fn render_app(
    store: Ref<Self::Model>,
    memo_caches: Rc<RefCell<MemoCache<RespoNode<Self::Action>>>>,
  ) -> Result<RespoNode<Self::Action>, String>;
  /// raq loop
  fn render_loop(&self) -> Result<(), String> {
    let mount_target = self.get_mount_target();

    // need to push store inside function to keep all in one thread
    let global_store = Rc::new(RefCell::new(Self::initial_model()));

    let memo_caches = init_memo_cache();

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
  /// init cache for memoization
  fn init_memo_cache(&self) -> Rc<RefCell<MemoCache<RespoNode<Self::Action>>>> {
    init_memo_cache::<RespoNode<Self::Action>>()
  }
}
