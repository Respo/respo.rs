## Respo in Rust

[![Respo Crate](https://img.shields.io/crates/v/respo?style=flat-square)](https://crates.io/crates/respo)

Docs(TODO) https://docs.rs/respo

> Reimagine [Respo.cljs](http://respo-mvc.org/) in Rust.

### Usage

A preview example:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Store {
  pub counted: i32,
  pub tasks: Vec<Task>,
  pub states: StatesTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
  pub id: String,
  pub done: bool,
  pub content: String,
  pub time: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActionOp {
  Increment,
  Decrement,
  StatesChange(Vec<String>, Option<Value>),
  AddTask(String, String),
  RemoveTask(String),
  UpdateTask(String, String),
  ToggleTask(String),
}

pub fn apply_action(store: &mut Store, op: ActionOp) -> Result<(), String> {
  match op {
    ActionOp::Increment => {
      store.counted += 1;
    }
    ActionOp::Decrement => {
      store.counted -= 1;
    }
    ActionOp::StatesChange(path, new_state) => {
      store.states = store.states.set_in(&path, new_state);
    }
    ActionOp::AddTask(id, content) => store.tasks.push(Task {
      id,
      content,
      time: 0.0,
      done: false,
    }),
    ActionOp::RemoveTask(id) => {
      store.tasks.retain(|task| task.id != id);
    }
    ActionOp::UpdateTask(id, content) => {
      let mut found = false;
      for task in &mut store.tasks {
        if task.id == id {
          task.content = content.to_owned();
          found = true;
        }
      }
      if !found {
        return Err(format!("task {} not found", id));
      }
    }
    ActionOp::ToggleTask(id) => {
      let mut found = false;
      for task in &mut store.tasks {
        if task.id == id {
          util::log!("change task {:?}", task);
          task.done = !task.done;
          found = true;
        }
      }
      if !found {
        return Err(format!("task {} not found", id));
      }
    }
  }
  Ok(())
}
```

```rust
#[wasm_bindgen(js_name = loadDemoApp)]
pub fn load_demo_app() -> JsValue {
  panic::set_hook(Box::new(console_error_panic_hook::hook));

  let mount_target = query_select_node(".app").expect("found mount target");

  // need to push store inside function to keep all in one thread
  let global_store = Rc::new(RefCell::new(Store {
    counted: 0,
    states: StatesTree::default(),
    tasks: vec![],
  }));

  let store_to_action = global_store.clone();
  let dispatch_action = move |op: ActionOp| -> Result<(), String> {
    // util::log!("action {:?} store, {:?}", op, store_to_action.borrow());
    let mut store = store_to_action.borrow_mut();
    apply_action(&mut store, op)?;

    // util::log!("store after action {:?}", store);
    Ok(())
  };

  render_node(
    mount_target,
    Box::new(move || -> Result<RespoNode<ActionOp>, String> {
      let store = global_store.borrow();
      let states = store.states.clone();

      // util::log!("global store: {:?}", store);

      Ok(
        div()
          .class(ui_global())
          .add_style(RespoStyle::default().padding(12.0).to_owned())
          .add_children([
            comp_counter(&states.pick("counter"), store.counted),
            comp_panel(&states.pick("panel"))?,
            comp_todolist(&states.pick("todolist"), &store.tasks)?,
          ])
          .to_owned(),
      )
    }),
    DispatchFn::new(dispatch_action),
  )
  .expect("rendering node");

  JsValue::NULL
}
```

### License

Apache License 2.0 .
