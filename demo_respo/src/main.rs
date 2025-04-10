extern crate console_error_panic_hook;

mod counter;
mod inner_text;
mod panel;
mod plugins;
mod store;
mod task;
mod todolist;

use std::cell::{Ref, RefCell};
use std::panic;
use std::rc::Rc;

use inner_text::comp_inner_text;
use respo::css::respo_style;
use respo::{contained_styles, space, RespoAction};
use web_sys::Node;

use respo::ui::ui_global;
use respo::{div, util::query_select_node};
use respo::{util, RespoApp, RespoNode, RespoStore};

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

  fn get_store(&self) -> &Rc<RefCell<Self::Model>> {
    &self.store
  }
  fn get_mount_target(&self) -> &web_sys::Node {
    &self.mount_target
  }

  fn pick_storage_key() -> &'static str {
    APP_STORE_KEY
  }

  fn dispatch(store_to_action: Rc<RefCell<Self::Model>>, op: <Self::Model as RespoStore>::Action) -> Result<(), String> {
    if let Some(intent) = op.detect_intent() {
      intent.update(store_to_action)
    } else {
      let mut store = store_to_action.borrow_mut();
      store.update(op)
    }
  }

  fn view(store: Ref<Self::Model>) -> Result<RespoNode<<Self::Model as RespoStore>::Action>, String> {
    let states = &store.states;
    // util::log!("global store: {:?}", store);

    Ok(
      div()
        .class(ui_global() + " " + &style_container())
        .style(respo_style().padding(12))
        .children([
          comp_counter(&states.pick("counter"), store.counted)?.to_node(),
          space(None, Some(80)).to_node(),
          comp_panel(&states.pick("panel"))?,
          comp_todolist(&states.pick("todolist"), &store.tasks)?.to_node(),
          space(None, Some(80)).to_node(),
          comp_plugins_demo(&states.pick("plugins-demo"))?.to_node(),
          space(None, Some(80)).to_node(),
          comp_inner_text(&states.pick("inner-text"))?.to_node(),
          space(None, Some(80)).to_node(),
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

contained_styles!(
  style_container,
  (
    Some("@media only screen and (max-width: 600px)".to_owned()),
    "&",
    respo_style().background_color(respo::css::CssColor::Hsl(0, 0, 95))
  )
);
