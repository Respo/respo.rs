use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

/// that can be stored in Respo's caching tree
pub trait RespoCacheable {
  fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone, Default)]
pub struct StatesTree {
  data: LocalState,
  cursor: Vec<String>,
  branches: HashMap<String, Rc<StatesTree>>,
}

impl StatesTree {
  // data
  pub fn load(&self) -> LocalState {
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
      data: LocalState(None),
      cursor: next_cursor,
      branches: HashMap::new(),
    }
  }

  /// returns a new tree
  pub fn set_in(&self, path: &[String], new_state: LocalState) -> Self {
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
        let branch = self.branches[&p0].clone();
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
pub struct LocalState(pub Option<Rc<dyn RespoCacheable>>);

impl Debug for LocalState {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "LocalStateWrapper {{}}")
  }
}

impl PartialEq for LocalState {
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

impl Eq for LocalState {}

impl LocalState {
  pub fn ref_into<T>(&self) -> Option<&T>
  where
    T: 'static + RespoCacheable,
  {
    // thanks to https://bennetthardwick.com/rust/downcast-trait-object/
    match &self.0 {
      Some(v) => Some(v.as_any().downcast_ref::<T>().expect("converting from local state")),
      None => None,
    }
  }

  pub fn ref_from<T>(data: Option<&T>) -> Self
  where
    T: 'static + RespoCacheable + Debug + Clone,
  {
    match data {
      Some(v) => {
        // thanks to https://users.rust-lang.org/t/why-i-can-use-dynamic-dispatch-dyn-mytrait-with-rc-but-not-refcell/49517/2
        // log_1(&format!("after convert: {:?}", l).into());
        let x: Rc<dyn RespoCacheable> = Rc::new(v.to_owned());
        LocalState(Some(x))
      }
      None => LocalState(None),
    }
  }
}
