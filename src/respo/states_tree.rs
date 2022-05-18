use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use web_sys::console::log_1;

pub trait LocalState {
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Default)]
pub struct StatesTree {
  data: LocalStateWrapper,
  cursor: Vec<String>,
  branches: HashMap<String, Rc<StatesTree>>,
}

impl StatesTree {
  // data
  pub fn load(&self) -> LocalStateWrapper {
    self.data.to_owned()
  }

  pub fn path(&self) -> Vec<String> {
    self.cursor.clone()
  }

  // pick a child branch as new cursor
  pub fn pick(&self, path: &str) -> StatesTree {
    let mut next_cursor = self.cursor.clone();
    next_cursor.push(path.to_owned());
    Self {
      data: LocalStateWrapper(None),
      cursor: next_cursor,
      branches: HashMap::new(),
    }
  }

  /// returns a new tree
  pub fn set_in(&self, path: &[String], new_state: LocalStateWrapper) -> Self {
    if path.is_empty() {
      Self {
        data: new_state,
        cursor: self.cursor.clone(),
        branches: self.branches.clone(),
      }
    } else {
      let (p_head, p_rest) = path.split_at(1);
      let p0 = p_head[0].to_owned();
      if self.branches.contains_key(&p0) {
        let branch = self.branches.get(&p0).unwrap().clone();
        let next_branch = branch.set_in(p_rest, new_state);

        let mut next = self.clone();
        next.branches.insert(p0, Rc::new(next_branch));
        next
      } else {
        let mut next = self.clone();
        next.branches.insert(p0, Rc::new(StatesTree::pick(self, &path[0])));
        next
      }
    }
  }
}

// a trick to put dyn trait object inside a struct
// thanks to https://users.rust-lang.org/t/how-to-add-a-trait-value-into-hashmap/6542/3
#[derive(Clone, Default)]
pub struct LocalStateWrapper(pub Option<Rc<dyn LocalState>>);

impl Debug for LocalStateWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "LocalStateWrapper {{}}")
  }
}

impl PartialEq for LocalStateWrapper {
  fn eq(&self, other: &Self) -> bool {
    match (&self.0, &other.0) {
      (None, None) => true,
      (Some(a), Some(b)) => {
        // TODO I don't have an idea for comparing such dynamic pointers
        #[allow(clippy::vtable_address_comparisons)]
        Rc::ptr_eq(a, b)
      }
      _ => false,
    }
  }
}

impl Eq for LocalStateWrapper {}

impl LocalStateWrapper {
  pub fn ref_into<T>(&self) -> Option<&T>
  where
    T: 'static + LocalState,
  {
    // thanks to https://bennetthardwick.com/rust/downcast-trait-object/
    match &self.0 {
      Some(v) => Some(v.as_any().downcast_ref::<T>().expect("converting from local state")),
      None => None,
    }
  }

  pub fn ref_from<T>(data: Option<&T>) -> Self
  where
    T: 'static + LocalState + Debug + Clone,
  {
    log_1(&format!("before convert: {:?}", data).into());
    match data {
      Some(v) => {
        // thanks to https://users.rust-lang.org/t/why-i-can-use-dynamic-dispatch-dyn-mytrait-with-rc-but-not-refcell/49517/2
        // log_1(&format!("after convert: {:?}", l).into());
        let x: Rc<dyn LocalState> = Rc::new(v.to_owned());
        LocalStateWrapper(Some(x))
      }
      None => LocalStateWrapper(None),
    }
  }
}
