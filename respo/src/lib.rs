//! Tiny **virtual DOM** library, compiles to WebAssembly, runs in browser, building interactive web apps with declarative code.
//!
//! This library is experimental, heavily influenced by React.js and ClojureScript.
//! Previously implementeded in [ClojureScript](https://github.com/Respo/respo.cljs) and [Calcit](https://github.com/Respo/respo.calcit).
//!
//! ![Respo Logo](https://cdn.tiye.me/logo/respo.png)
//!
//! To build UI:
//!
//! - there's Virtual DOM, although simplified, still flexible for declarative UI
//! - CSS with Rust macros, I call it "CSS in Rust"
//! - Effects, flow from data to DOM, for patching DOM manually on data change
//! - `respo::ui` provides basic style. Also try Modal, dialog, drawer components.
//!
//! To manage states:
//!
//! - Rust enum and pattern matching it really nice for Elm-style action dispatching
//! - global states tree and cursor to maintain states, may not be familiar but still being handy
//! - you may also write shared component like a "plugin" to manage states.
//!
//! To optimize:
//!
//! - components and elements are in functions, available for [memoize](https://crates.io/crates/memoize)
//! - well, it's Rust, you can do more...
//!
//! Meanwhile it does not support React features such as:
//!
//! - ❌ updating data from render. Respo enforces "unidirectional data flow". That's not allowed
//! - ❌ hooks API and context. Respo uses plain functions without tricky internal states
//! - ...does not have many other advanced features from React
//!
//! Rust and WebAssembly lacks tricks for hot reloading,
//! it's suggested to use [trunk](https://github.com/trunk-rs/trunk) to edit and reload the project during development.
//! App states including components states can be saved to local storage and reloaded.
//!
//! To start project, create your structs to implement traits:
//!
//! - `RespoStore` for global states and states tree, and `RespoAction` for updating
//! - `RespoApp` for MVC overview of the app, and more views, bind events
//!
//! say app is called `app`, you start app with `app.render_loop()`.
//! Check [Workflow](https://github.com/Respo/respo-rust-workflow/tree/c7cc0c0/src) for a working example.

mod app;
pub mod states_tree;

pub(crate) mod node;
pub mod ui;

pub use node::element::alias::*;
pub use node::*;

pub use app::{util, RespoApp, RespoStore};
