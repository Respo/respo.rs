//! A tiny framework around a **virtual DOM** library, compiles to WebAssembly, runs in the browser, declarative UI for building interactive web apps.
//!
//! Original design was [Respo.cljs](http://respo-mvc.org/), which is heavily influenced by React.js and ClojureScript.
//! And this module is still "experimental" since WebAssembly lacks of hot reloading.
//!
//! It features:
//!
//! - virtual DOM(however simplified in list diffing)
//! - components declaration with functions
//! - globals states with Store and Actions dispatching
//! - states tree with nested states(inherited from Respo.cljs , might be akward)
//! - CSS in Rust macros
//! - basic component effects of `Mounted, WillUpdate, Updated, WillUnmount`
//! - macros to memoize component functions(although look clumsy)
//!
//! Meanwhile it does not include features like:
//!
//! - ❌ macros for JSX syntax. Respo prefer types over tags
//! - ❌ updating component states in lifecycle. Respo enforces "unidirectional data flow"
//! - ❌ React-like hooks. Respo uses plain functions without tricky internal states
//! - ❌ Hot reloading. Not working in WebAssembly so far

mod memof1;
mod respo;

pub mod alerts;
pub mod ui;

pub use crate::respo::*;
pub use memof1::*;
