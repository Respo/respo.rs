use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum CssSize {
  Auto,
  Px(f32),
  Percent(f32),
  Vw(f32),
  Vh(f32),
  /// may be calc or something
  Custom(String),
}

/// extends built-in usize for CSS size
pub trait ConvertRespoCssSize {
  fn px(self) -> CssSize;
  fn percent(self) -> CssSize;
  fn vw(self) -> CssSize;
  fn vh(self) -> CssSize;
}

impl ConvertRespoCssSize for i32 {
  fn px(self) -> CssSize {
    CssSize::Px(self as f32)
  }
  fn percent(self) -> CssSize {
    CssSize::Percent(self as f32)
  }
  fn vw(self) -> CssSize {
    CssSize::Vw(self as f32)
  }
  fn vh(self) -> CssSize {
    CssSize::Vh(self as f32)
  }
}

impl Display for CssSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Auto => write!(f, "auto"),
      Self::Px(v) => write!(f, "{}px", v),
      Self::Percent(v) => write!(f, "{}%", v),
      Self::Vw(v) => write!(f, "{}vw", v),
      Self::Vh(v) => write!(f, "{}vh", v),
      Self::Custom(v) => write!(f, "{}", v),
    }
  }
}

/// initially added to support passing both i32 and f32 to methods
pub enum CssPx {
  Px(f32),
}

impl From<f32> for CssPx {
  fn from(v: f32) -> Self {
    Self::Px(v)
  }
}

impl From<i32> for CssPx {
  fn from(v: i32) -> Self {
    Self::Px(v as f32)
  }
}

impl Display for CssPx {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Px(v) => write!(f, "{}px", v),
    }
  }
}
