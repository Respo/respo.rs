use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use serde_json::Value;

/// dictionary of memoized functions
pub type MemoCache<T> = Rc<RefCell<HashMap<usize, HashMap<String, (Vec<Value>, T)>>>>;

/// internal function for handling `memo1_call_by`
pub fn internal_memof1_call_by<F, T>(caches: MemoCache<T>, address: usize, key: String, args: Vec<Value>, f: F) -> Result<T, String>
where
  T: Debug + Clone,
  F: Fn() -> Result<T, String>,
{
  let mut cache = caches.borrow_mut();
  let f_dict = cache.entry(address).or_insert_with(HashMap::new);
  let value = f_dict.get(&key);
  if let Some((old_args, v)) = value {
    if old_args == &args {
      Ok(v.to_owned())
    } else {
      let v = f()?;
      f_dict.insert(key, (args, v.to_owned()));
      Ok(v)
    }
  } else {
    let v = f()?;
    f_dict.insert(key, (args, v.to_owned()));
    Ok(v)
  }
}

/// exteand to code that interacts with memo cache, that order of args:
/// ```ignore
/// memo1_call_by!($f, $memo_caches, $key, $args...)
/// ```
/// for example:
/// ```ignore
/// memo1_call_by!(comp_task, memo_caches.to_owned(), task.id.to_owned(), &states.pick(&task.id), task)?
/// ```
/// its expanding result would be like:
/// ```ignore
/// internal_memof1_call_by(
///   memo_caches.to_owned(),
///   comp_task as usize,
///   task.id.to_owned(),
///   vec![cast_into_json(states.pick(&task.id)), cast_into_json(task)],
///   move || comp_task(m.to_owned(), &states.pick(&task.id), task),
/// )
/// ```
#[macro_export]
macro_rules! memo1_call_by {
  // 1 argument
  ($f:ident, $cache:expr, $key:expr, $arg1:expr) => {
    $crate::internal_memof1_call_by(
      $cache.to_owned(),
      $f as usize,
      $key.to_owned(),
      vec![$crate::util::cast_into_json($arg1)],
      move || $f($cache.to_owned(), $arg1),
    )
  };
  // to allow optional trailing comma
  ($f:ident, $cache:expr, $key:expr, $arg1:expr,) => {
    $crate::memo1_call_by!($f, $cache, $key, $arg1);
  };
  // 2 arguments
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr) => {
    $crate::internal_memof1_call_by(
      $cache.to_owned(),
      $f as usize,
      $key.to_owned(),
      vec![$crate::util::cast_into_json($arg1), $crate::util::cast_into_json($arg2)],
      move || $f($cache.to_owned(), $arg1, $arg2),
    )
  };
  // to allow optional trailing comma
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr,) => {
    $crate::memo1_call_by!($f, $cache, $key, $arg1, $arg2);
  };
  // 3 arguments
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
    $crate::internal_memof1_call_by(
      $cache.to_owned(),
      $f as usize,
      $key.to_owned(),
      vec![
        $crate::util::cast_into_json($arg1),
        $crate::util::cast_into_json($arg2),
        $crate::util::cast_into_json($arg3),
      ],
      move || $f($cache.to_owned(), $arg1, $arg2, $arg3),
    )
  };
  // to allow optional trailing comma
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr,) => {
    $crate::memo1_call_by!($f, $cache, $key, $arg1, $arg2, $arg3);
  };
  // 4 arguments
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
    $crate::internal_memof1_call_by(
      $cache.to_owned(),
      $f as usize,
      $key.to_owned(),
      vec![
        $crate::util::cast_into_json($arg1),
        $crate::util::cast_into_json($arg2),
        $crate::util::cast_into_json($arg3),
        $crate::util::cast_into_json($arg4),
      ],
      move || $f($cache.to_owned(), $arg1, $arg2, $arg3, $arg4),
    )
  };
  // to allow optional trailing comma
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr,) => {
    $crate::memo1_call_by!($f, $cache, $key, $arg1, $arg2, $arg3, $arg4);
  };
  // 5 arguments
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
    $crate::internal_memof1_call_by(
      $cache.to_owned(),
      $f as usize,
      $key.to_owned(),
      vec![
        $crate::util::cast_into_json($arg1),
        $crate::util::cast_into_json($arg2),
        $crate::util::cast_into_json($arg3),
        $crate::util::cast_into_json($arg4),
        $crate::util::cast_into_json($arg5),
      ],
      move || $f($cache.to_owned(), $arg1, $arg2, $arg3, $arg4, $arg5),
    )
  };
  // to allow optional trailing comma
  ($f:ident, $cache:expr, $key:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
    $crate::memo1_call_by!($f, $cache, $key, $arg1, $arg2, $arg3, $arg4, $arg5);
  };
}
pub use memo1_call_by;
