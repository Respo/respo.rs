//! A tiny framework around a **virtual DOM** library, compiles to WebAssembly, runs in the browser, declarative UI for building interactive web apps.
//!
//! Original design was [Respo.cljs](http://respo-mvc.org/), which is heavily influenced by React.js and ClojureScript.
//! This module is experimental since WebAssembly lacks of hot reloading.
//!
//! It features:
//!
//! - virtual DOM(however simplified in list diffing)
//! - components declaration with functions
//! - globals states with Store and Actions dispatching
//! - states tree with nested states(inherited from Respo.cljs , might be strange)
//! - CSS with Rust macros
//! - basic component effects of `Mounted, WillUpdate, Updated, WillUnmount`
//! - okay to [memoize](https://crates.io/crates/memoize) component functions
//!
//! Meanwhile it does not include features like:
//!
//! - ❌ macros for JSX syntax. Respo prefer types over tags
//! - ❌ updating component states in lifecycle. Respo enforces "unidirectional data flow"
//! - ❌ React-like hooks. Respo uses plain functions without tricky internal states
//! - ❌ Hot swapping. Respo.rs reload on edit and loads previous states from local storage.

mod app;
pub mod states_tree;

pub(crate) mod node;
pub mod ui;

pub use node::css::*;
pub use node::element::alias::*;
pub use node::*;

pub use app::renderer::*;

pub use app::{util, RespoApp, RespoStore};
