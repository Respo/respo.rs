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

use web_sys::Node;

use respo::ui::ui_global;
use respo::{div, util::query_select_node};
use respo::{util, RespoApp, RespoNode, RespoStore, RespoStyle};

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
}

impl RespoApp for App {
  type Model = Store;
  type Action = ActionOp;

  fn get_store(&self) -> &Rc<RefCell<Self::Model>> {
    &self.store
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }

  fn pick_storage_key() -> &'static str {
    APP_STORE_KEY
  }

  fn dispatch(store: &mut RefMut<Self::Model>, op: Self::Action) -> Result<(), String> {
    store.update(op)
  }

  fn view(store: Ref<Self::Model>) -> Result<RespoNode<Self::Action>, String> {
    let states = &store.states;
    // util::log!("global store: {:?}", store);

    Ok(
      div()
        .class(ui_global())
        .style(RespoStyle::default().padding(12.0))
        .children([
          comp_counter(&states.pick("counter"), store.counted)?.to_node(),
          comp_panel(&states.pick("panel"))?,
          comp_todolist(&states.pick("todolist"), &store.tasks)?.to_node(),
          comp_plugins_demo(&states.pick("plugins-demo"))?.to_node(),
        ])
        .to_node(),
    )
  }
}

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(Store::default())),
  };

  app.try_load_storage().expect("load storage");
  app.backup_model_beforeunload().expect("backup model beforeunload");

  util::log!("store: {:?}", app.store);

  app.render_loop().expect("app render");
}
