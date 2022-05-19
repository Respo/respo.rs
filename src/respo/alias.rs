//! provide some frequestly used aliases,
//! for the rest, use `RespoNode::make_tag(tag_name)` to create you own.

use std::fmt::Debug;

use super::primes::RespoNode;

#[macro_export]
macro_rules! declare_tag {
  ( $name:ident ) => {
    #[allow(dead_code)]
    pub fn $name<T>() -> RespoNode<T>
    where
      T: Debug + Clone,
    {
      RespoNode::make_tag(stringify!($name))
    }
  };
}

declare_tag!(div);
declare_tag!(header);
declare_tag!(section);
declare_tag!(footer);
declare_tag!(span);
declare_tag!(input);
declare_tag!(link);
declare_tag!(button);
declare_tag!(pre);
declare_tag!(img);
declare_tag!(video);
declare_tag!(code);
declare_tag!(a);
declare_tag!(h1);
declare_tag!(h2);
declare_tag!(h3);
declare_tag!(h4);
declare_tag!(blockquote);
