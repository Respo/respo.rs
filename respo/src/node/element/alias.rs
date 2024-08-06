//! provide some frequestly used aliases,
//! for the rest, use `RespoNode::make_tag(tag_name)` to create you own.

use std::fmt::Debug;

use super::RespoElement;
use crate::css::{respo_style, ConvertRespoCssSize};

/// a macro for creating a function with a named node
/// ```ignore
/// declare_tag!(div, "about `<div/>`");
/// ```
#[macro_export]
macro_rules! declare_tag {
  ( $name:ident, $doc: expr) => {
    #[doc=$doc]
    #[allow(dead_code)]
    pub fn $name<T>() -> RespoElement<T>
    where
      T: Debug + Clone,
    {
      $crate::node::RespoElement::named(stringify!($name))
    }
  };
}

declare_tag!(div, "`<div/>`");
declare_tag!(header, "`<header/>`");
declare_tag!(section, "`<section/>`");
declare_tag!(footer, "`<footer/>`");
declare_tag!(br, "`<br/>`");
declare_tag!(span, "`<span/>`");
declare_tag!(input, "`<input/>`");
declare_tag!(textarea, "`<input/>`");
declare_tag!(link, "`<link/>`");
declare_tag!(button, "`<button/>`");
declare_tag!(pre, "`<pre/>`");
declare_tag!(img, "`<img/>`");
declare_tag!(video, "`<video/>`");
declare_tag!(code, "`<code/>`");
declare_tag!(a, "`<a/>`");
declare_tag!(h1, "`<h1/>`");
declare_tag!(h2, "`<h2/>`");
declare_tag!(h3, "`<h3/>`");
declare_tag!(h4, "`<h4/>`");
declare_tag!(blockquote, "`<blockquote/>`");
declare_tag!(ul, "`<ul/>`");
declare_tag!(li, "`<li/>`");
declare_tag!(label, "`<label/>`");
declare_tag!(canvas, "`<canvas/>`");

/// special function to return `<div/>` with width/height that can be used as a space
pub fn space<T>(w: Option<i32>, h: Option<i32>) -> RespoElement<T>
where
  T: Clone + Debug,
{
  match (w, h) {
    (Some(wv), Some(hv)) => div().style(
      respo_style()
        .width(wv.px())
        .height(hv.px())
        .display(crate::node::css::CssDisplay::InlineBlock),
    ),
    (Some(wv), None) => span().style(respo_style().width(wv.px()).display(crate::node::css::CssDisplay::InlineBlock)),
    (None, Some(hv)) => div().style(
      respo_style()
        .height(hv.px())
        .width(1.px())
        .display(crate::node::css::CssDisplay::Block),
    ),
    (None, None) => span().style(
      respo_style()
        .width(8.px())
        .height(8.px())
        .display(crate::node::css::CssDisplay::InlineBlock),
    ),
  }
}
