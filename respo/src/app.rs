pub(crate) mod diff;
pub(crate) mod patch;
pub(crate) mod renderer;

use std::{
  cell::{Ref, RefCell},
  fmt::Debug,
  rc::Rc,
};

pub mod util;

use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{BeforeUnloadEvent, Node};

use renderer::render_node;

use crate::{
  node::{DispatchFn, RespoAction, RespoNode},
  states_tree::{RespoStatesTree, RespoUpdateState},
};

const RESPO_APP_STORE_KEY: &str = "respo_app_respo_store_default";

/// A template for a Respo app
pub trait RespoApp {
  /// a type of the store, with a place for states tree
  type Model: RespoStore + Debug + Clone + 'static;

  /// simulating pure function updates to the model, but actually it's mutations
  fn dispatch(store: Rc<RefCell<Self::Model>>, action: <Self::Model as RespoStore>::Action) -> Result<(), String>;

  /// used when saving to local storage
  fn pick_storage_key() -> &'static str {
    RESPO_APP_STORE_KEY
  }

  /// bridge to mount target
  fn get_mount_target(&self) -> &Node;
  /// bridge to store
  fn get_store(&self) -> &Rc<RefCell<Self::Model>>;

  /// default interval in milliseconds, by default 100ms,
  /// pass `None` to use raq directly, pass `Some(200)` to redice cost
  fn get_loop_delay() -> Option<i32> {
    Some(100)
  }

  /// DSL for building a view
  fn view(store: Ref<Self::Model>) -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String>;
  /// start a requestAnimationFrame loop for rendering updated store
  fn render_loop(&self) -> Result<(), String> {
    let mount_target = self.get_mount_target();
    let global_store = self.get_store();

    // let store_to_action = global_store.to_owned();
    let dispatch_action = {
      let store_to_action = global_store.to_owned();
      move |op: <Self::Model as RespoStore>::Action| -> Result<(), String> {
        // util::log!("action {:?} store, {:?}", op, store_to_action.borrow());

        Self::dispatch(store_to_action.to_owned(), op)?;
        // util::log!("store after action {:?}", store);
        Ok(())
      }
    };

    render_node(
      mount_target.to_owned(),
      Box::new({
        let store_to_action = global_store.to_owned();
        move || store_to_action.borrow().to_owned()
      }),
      Box::new({
        let store = global_store.to_owned();
        move || -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String> {
          // util::log!("global store: {:?}", store);

          Self::view(store.borrow())
        }
      }),
      DispatchFn::new(dispatch_action),
      Self::get_loop_delay(),
    )
    .unwrap_or_else(|e| {
      util::error_log!("render loop error: {:?}", e);
    });

    Ok(())
  }

  /// backup store to local storage before unload
  fn backup_model_beforeunload(&self) -> Result<(), String> {
    let window = web_sys::window().expect("window");
    let storage = match window.local_storage() {
      Ok(Some(storage)) => storage,
      _ => return Err("Failed to access local storage".to_owned()),
    };
    let beforeunload = Closure::wrap(Box::new({
      let p = Self::pick_storage_key();
      let store = self.get_store().to_owned();
      move |_e: BeforeUnloadEvent| {
        let content = store.as_ref().borrow().to_string();
        // util::log!("before unload {} {}", p, content);
        storage.set_item(p, &content).expect("save storage");
      }
    }) as Box<dyn FnMut(BeforeUnloadEvent)>);
    window.set_onbeforeunload(Some(beforeunload.as_ref().unchecked_ref()));
    beforeunload.forget();
    Ok(())
  }

  fn try_load_storage(&self) -> Result<(), String> {
    let window = web_sys::window().expect("window");
    let storage = match window.local_storage() {
      Ok(Some(storage)) => storage,
      _ => return Err("Failed to access local storage".to_owned()),
    };

    let key = Self::pick_storage_key();
    match storage.get_item(key) {
      Ok(Some(s)) => match Self::Model::try_from_string(&s) {
        Ok(s) => {
          let store = self.get_store();
          *store.borrow_mut() = s;
        }
        Err(e) => {
          util::error_log!("error: {:?}", e);
        }
      },
      _ => {
        util::log!("no storage");
      }
    }
    Ok(())
  }
}

/// it has a states tree inside, and it does update itself
pub trait RespoStore {
  type Action: Debug + Clone + RespoAction;
  fn update(&mut self, op: Self::Action) -> Result<(), String>;

  /// a way to load states tree
  fn get_states(&mut self) -> &mut RespoStatesTree;

  /// public API for updating states tree
  fn update_states(&mut self, op: RespoUpdateState) {
    self.get_states().set_in_mut(op);
  }

  /// for backup
  fn to_string(&self) -> String;

  /// load from backup
  fn try_from_string(s: &str) -> Result<Self, String>
  where
    Self: Sized;
}
