//! A tiny frameworkd around a Virtual DOM library, migrated from Respo.cljs .

mod app;
mod respo;

pub use app::load_demo_app;
pub use respo::render_element;
