extern crate console_error_panic_hook;

mod counter;
mod panel;
mod plugins;
mod store;
mod task;
mod todolist;

use std::cell::{Ref, RefCell, RefMut};
use std::panic;
use std::rc::Rc;

use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{BeforeUnloadEvent, Node};

use respo::ui::ui_global;
use respo::{div, util::query_select_node};
use respo::{MemoCache, RespoApp, RespoNode, RespoStore, RespoStyle};

use self::counter::comp_counter;
pub use self::store::ActionOp;
use self::store::*;
use self::todolist::comp_todolist;
use panel::comp_panel;
use plugins::comp_plugins_demo;

const APP_STORE_KEY: &str = "demo_respo_store";

struct App {
  store: Rc<RefCell<Store>>,
  mount_target: Node,
  memo_caches: MemoCache<RespoNode<ActionOp>>,
}

impl RespoApp for App {
  type Model = Store;
  type Action = ActionOp;

  fn get_store(&self) -> Rc<RefCell<Self::Model>> {
    self.store.clone()
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }
  fn get_memo_caches(&self) -> MemoCache<RespoNode<Self::Action>> {
    self.memo_caches.to_owned()
  }

  fn dispatch(store: &mut RefMut<Self::Model>, op: Self::Action) -> Result<(), String> {
    store.update(op)
  }

  fn view(store: Ref<Self::Model>, memo_caches: MemoCache<RespoNode<Self::Action>>) -> Result<RespoNode<Self::Action>, String> {
    let states = &store.states;
    // util::log!("global store: {:?}", store);

    Ok(
      div()
        .class(ui_global())
        .style(RespoStyle::default().padding(12.0).to_owned())
        .children([
          comp_counter(&states.pick("counter"), store.counted)?,
          comp_panel(&states.pick("panel"))?,
          comp_todolist(memo_caches, &states.pick("todolist"), &store.tasks)?,
          comp_plugins_demo(&states.pick("plugins-demo"))?,
        ])
        .to_owned(),
    )
  }
}

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  // util::log!("todo");
  let window = web_sys::window().expect("window");
  let storage = window.local_storage().expect("get storage").expect("unwrap storage");

  let prev_store: Option<Store> = match storage.get_item(APP_STORE_KEY) {
    Ok(Some(s)) => match serde_json::from_str(&s) {
      Ok(s) => Some(s),
      Err(e) => {
        respo::util::log!("error: {:?}", e);
        None
      }
    },
    Ok(None) => None,
    Err(_e) => None,
  };

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(prev_store.unwrap_or_default())),
    memo_caches: MemoCache::default(),
  };

  let store2 = app.store.clone();
  let beforeunload = Closure::wrap(Box::new(move |_e: BeforeUnloadEvent| {
    respo::util::log!("before unload.");
    let s: &Store = &store2.borrow();
    storage
      .set_item(APP_STORE_KEY, &serde_json::to_string(s).expect("to json"))
      .expect("save storage");
  }) as Box<dyn FnMut(BeforeUnloadEvent)>);
  window.set_onbeforeunload(Some(beforeunload.as_ref().unchecked_ref()));
  beforeunload.forget();

  app.render_loop().expect("app render");
}
