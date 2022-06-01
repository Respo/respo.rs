## Respo in Rust

[![Respo Crate](https://img.shields.io/crates/v/respo?style=flat-square)](https://crates.io/crates/respo)

> tiny toy virtual DOM based framework for Rust.
>
> Status: experimenting, unhappy without HMR.
>
> Respo was initially designed to work in a dynamic language with persistent data and HMR(hot code replacement), which is dramatically different from Rust. So this is more like an experiment.

Docs https://docs.rs/respo

### Usage

A preview example, to delare a store:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
  pub states: StatesTree,
  // TODO you app data
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
  // TODO
  StatesChange(Vec<String>, MaybeState),
}

impl RespoAction for ActionOp {
  fn wrap_states_action(cursor: &[String], a: MaybeState) -> Self {
    Self::StatesChange(cursor.to_vec(), a)
  }
}

impl RespoStore for Store {
  type Action = ActionOp;

  fn get_states(&self) -> StatesTree {
    self.states.to_owned()
  }
  fn update(&mut self, op: Self::Action) -> Result<(), String> {
    match op {
      // TODO
    }
    Ok(())
  }
}
```

To declare an app:

```rust
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
        .add_style(RespoStyle::default().padding(12.0).to_owned())
        .add_children([
          comp_counter(&states.pick("counter"), store.counted)?,
          comp_panel(&states.pick("panel"))?,
          comp_todolist(memo_caches, &states.pick("todolist"), &store.tasks)?,
        ])
        .to_owned(),
    )
  }
}
```

mount app:

```rust
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
```

CSS-in-Rust:

```rust
static_styles!(
  style_remove_button,
  (
    "$0".to_owned(),
    RespoStyle::default()
      .width(CssSize::Px(16.0))
      .height(CssSize::Px(16.0))
      .margin(4.)
      .cursor("pointer".to_owned())
      .margin4(0.0, 0.0, 0.0, 16.0)
      .color(CssColor::Hsl(0, 90, 90)),
  ),
  ("$0:hover".to_owned(), RespoStyle::default().color(CssColor::Hsl(0, 90, 80))),
);
```

For components, read code in `src/app/`, they are just variants like `RespoNode::Component(..)`. It may be sugared in the future, not decided yet.

### License

Apache License 2.0 .
