extern crate console_error_panic_hook;

mod counter;
mod panel;
mod store;
mod task;
mod todolist;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::{panic, vec};

use web_sys::Node;

use respo::ui::ui_global;
use respo::{div, util::query_select_node, StatesTree};
use respo::{MemoCache, RespoApp, RespoNode, RespoStore, RespoStyle};

use self::counter::comp_counter;
pub use self::store::ActionOp;
use self::store::*;
use self::todolist::comp_todolist;
use panel::comp_panel;

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
        ])
        .to_owned(),
    )
  }
}

fn main() {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let app = App {
    mount_target: query_select_node(".app").expect("mount target"),
    store: Rc::new(RefCell::new(Store {
      counted: 0,
      states: StatesTree::default(),
      tasks: vec![],
    })),
    memo_caches: MemoCache::default(),
  };

  app.render_loop().expect("app render");
}
