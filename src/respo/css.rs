use std::{
  collections::HashMap,
  fmt::{self, Display, Formatter},
};

/// it provides ADT interfaces as APIs, but internally it's maintained dynamically.
/// it's easily diffed in a hashmap.
/// and when it's sent to JS APIs, it's still in strings, which is also dynamic.
/// TODO order of rules might matter in edge cases
#[derive(Debug, Clone, PartialEq, Default)]
pub struct RespoStyle(pub HashMap<String, String>);

impl RespoStyle {
  pub fn add(&mut self, rule: RespoStyleRule) -> &mut Self {
    let (property, value) = rule.get_pair();
    self.0.insert(property, value);
    self
  }
  pub fn insert(&mut self, property: String, value: String) -> &mut Self {
    self.0.insert(property, value);
    self
  }
}

impl Display for RespoStyle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (key, value) in self.0.iter() {
      write!(f, "{}:{};", key, value)?;
    }
    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RespoStyleRule {
  // box styles
  Width(RespoSize),
  Height(RespoSize),
  Margin(f32),
  Margin4(f32, f32, f32, f32),
  Padding(f32),
  Padding4(f32, f32, f32, f32),
  Border(f32, RespoBorderStyle, RespoColor),
  Outline(f32, RespoBorderStyle, RespoColor),
  BoxShadow(f32, f32, f32, f32, RespoColor),
  BorderRadius(f32),
  Overflow(RespoOverflow),
  MaxWidth(RespoSize),
  MaxHeight(RespoSize),
  Opacity(f32),
  // background
  BackgroundColor(RespoColor),
  BackgroundImage(String),
  BackgroundSize(RespoBackgroundSize),
  BackgroundFilter(String),
  // text styles
  Color(RespoColor),
  FontFamily(String),
  FontSize(f32),
  FontStyle(RespoFontStyle),
  TextShadow(f32, f32, f32, RespoColor),
  LineHeight(RespoLineHeight),
  VerticalAlign(RespoVerticalAlign),
  TextOverflow(RespoTextOverflow),
  Cursor(String),
  // flex styles
  Display(RespoDisplay),
  FlexDirection(RespoFlexDirection),
  FlexWrap(RespoFlexWrap),
  JustifyContent(RespoFlexJustifyContent),
  AlignItems(RespoFlexAlignItems),
  // positions
  Position(RespoPosition),
  Top(RespoSize),
  Left(RespoSize),
  Right(RespoSize),
  Bottom(RespoSize),
  ZIndex(i32),
  // transform
  Transform(RespoTransform),
  TransitionDuration(f32),
  TransitionProperty(String),
  TransitionTimingFunction(RespoTimingFunction),
  TransitionDelay(f32),
}

impl Display for RespoStyleRule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let pair = self.get_pair();
    f.write_str(&pair.0)?;
    f.write_str(&pair.1)
  }
}

impl RespoStyleRule {
  pub fn get_pair(&self) -> (String, String) {
    match self {
      RespoStyleRule::Width(size) => ("width".to_owned(), size.to_string()),
      RespoStyleRule::Height(size) => ("height".to_owned(), size.to_string()),
      RespoStyleRule::Margin(margin) => ("margin".to_owned(), format!("{margin}px")),
      RespoStyleRule::Margin4(top, right, bottom, left) => ("margin".to_owned(), format!("{top}px {right}px {bottom}px {left}px")),
      RespoStyleRule::Padding(padding) => ("padding".to_owned(), format!("{padding}px")),
      RespoStyleRule::Padding4(top, right, bottom, left) => ("padding".to_owned(), format!("{top}px {right}px {bottom}px {left}px")),
      RespoStyleRule::Border(width, style, color) => ("border".to_owned(), format!("{width}px {style} {color}")),
      RespoStyleRule::Outline(width, style, color) => ("outline".to_owned(), format!("{width}px {style} {color}")),
      RespoStyleRule::BoxShadow(x, y, blur, spread, color) => {
        ("box-shadow".to_owned(), format!("{x}px {y}px {blur}px {spread}px {color}"))
      }
      RespoStyleRule::BorderRadius(radius) => ("border-radius".to_owned(), format!("{radius}px")),
      RespoStyleRule::Overflow(overflow) => ("overflow".to_owned(), overflow.to_string()),
      RespoStyleRule::MaxWidth(size) => ("max-width".to_owned(), format!("{size}px")),
      RespoStyleRule::MaxHeight(size) => ("max-height".to_owned(), format!("{size}px")),
      RespoStyleRule::Opacity(opacity) => ("opacity".to_owned(), opacity.to_string()),
      RespoStyleRule::BackgroundColor(color) => ("background-color".to_owned(), color.to_string()),
      RespoStyleRule::BackgroundImage(image) => ("background-image".to_owned(), image.to_string()),
      RespoStyleRule::BackgroundSize(size) => ("background-size".to_owned(), size.to_string()),
      RespoStyleRule::BackgroundFilter(filter) => ("background-filter".to_owned(), filter.to_string()),
      RespoStyleRule::Color(color) => ("color".to_owned(), color.to_string()),
      RespoStyleRule::FontFamily(family) => ("font-family".to_owned(), family.to_string()),
      RespoStyleRule::FontSize(size) => ("font-size".to_owned(), format!("{size}px")),
      RespoStyleRule::FontStyle(style) => ("font-style".to_owned(), style.to_string()),
      RespoStyleRule::TextShadow(x, y, blur, color) => ("text-shadow".to_owned(), format!("{x}px {y}px {blur}px {color}px")),
      RespoStyleRule::LineHeight(line_height) => ("line-height".to_owned(), line_height.to_string()),
      RespoStyleRule::VerticalAlign(align) => ("vertical-align".to_owned(), align.to_string()),
      RespoStyleRule::TextOverflow(overflow) => ("text-overflow".to_owned(), overflow.to_string()),
      RespoStyleRule::Display(display) => ("display".to_owned(), display.to_string()),
      RespoStyleRule::FlexDirection(direction) => ("flex-direction".to_owned(), direction.to_string()),
      RespoStyleRule::FlexWrap(wrap) => ("flex-wrap".to_owned(), wrap.to_string()),
      RespoStyleRule::JustifyContent(content) => ("justify-content".to_owned(), content.to_string()),
      RespoStyleRule::AlignItems(align) => ("align-items".to_owned(), align.to_string()),
      RespoStyleRule::Position(position) => ("position".to_owned(), position.to_string()),
      RespoStyleRule::Top(size) => ("top".to_owned(), size.to_string()),
      RespoStyleRule::Left(size) => ("left".to_owned(), size.to_string()),
      RespoStyleRule::Right(size) => ("right".to_owned(), size.to_string()),
      RespoStyleRule::Bottom(size) => ("bottom".to_owned(), size.to_string()),
      RespoStyleRule::ZIndex(index) => ("z-index".to_owned(), index.to_string()),
      RespoStyleRule::Transform(transform) => ("transform".to_owned(), transform.to_string()),
      RespoStyleRule::TransitionDuration(duration) => ("transition-duration".to_owned(), duration.to_string()),
      RespoStyleRule::TransitionProperty(property) => ("transition-property".to_owned(), property.to_string()),
      RespoStyleRule::TransitionTimingFunction(timing_function) => {
        ("transition-timing-function".to_owned(), timing_function.to_string())
      }
      RespoStyleRule::TransitionDelay(delay) => ("transition-delay".to_owned(), delay.to_string()),
      RespoStyleRule::Cursor(v) => ("cursor".to_owned(), v.to_string()),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RespoSize {
  Auto,
  Px(f32),
  Percent(f32),
  Vw(f32),
  Wh(f32),
  /// may be calc or something
  Custom(String),
}

impl Display for RespoSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoSize::Auto => write!(f, "auto"),
      RespoSize::Px(v) => write!(f, "{}px", v),
      RespoSize::Percent(v) => write!(f, "{}%", v),
      RespoSize::Vw(v) => write!(f, "{}vw", v),
      RespoSize::Wh(v) => write!(f, "{}wh", v),
      RespoSize::Custom(v) => write!(f, "{}", v),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoPosition {
  Static,
  Relative,
  Absolute,
  Fixed,
}

impl Display for RespoPosition {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        RespoPosition::Static => "static",
        RespoPosition::Relative => "relative",
        RespoPosition::Absolute => "absolute",
        RespoPosition::Fixed => "fixed",
      }
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RespoColor {
  Hsla(f32, f32, f32, f32),
  /// HCL color, to support later
  Hcla(f32, f32, f32, f32),
  Rgba(u8, u8, u8, u8),
  Hex(u8),
  Red,
  Green,
  Blue,
  White,
  Black,
  Gray,
  Yellow,
  Purple,
  Cyan,
  Orange,
  Pink,
}

impl Display for RespoColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        RespoColor::Hsla(h, s, l, a) => format!("hsla({}, {}%, {}%, {})", h, s, l, a),
        RespoColor::Hcla(h, c, l, a) => format!("hcla({}, {}, {}, {})", h, c, l, a), // TODO
        RespoColor::Rgba(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
        RespoColor::Hex(h) => format!("#{:02x}", h),
        RespoColor::Red => "red".to_string(),
        RespoColor::Green => "green".to_string(),
        RespoColor::Blue => "blue".to_string(),
        RespoColor::White => "white".to_string(),
        RespoColor::Black => "black".to_string(),
        RespoColor::Gray => "gray".to_string(),
        RespoColor::Yellow => "yellow".to_string(),
        RespoColor::Purple => "purple".to_string(),
        RespoColor::Cyan => "cyan".to_string(),
        RespoColor::Orange => "orange".to_string(),
        RespoColor::Pink => "pink".to_string(),
      }
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RespoLineHeight {
  Em(f32),
  Px(f32),
}

impl Display for RespoLineHeight {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoLineHeight::Em(v) => write!(f, "{}em", v),
      RespoLineHeight::Px(v) => write!(f, "{}px", v),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoFontStyle {
  Normal,
  Italic,
  Oblique,
}

impl Display for RespoFontStyle {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoFontStyle::Normal => write!(f, "normal"),
      RespoFontStyle::Italic => write!(f, "italic"),
      RespoFontStyle::Oblique => write!(f, "oblique"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoDisplay {
  Flex,
  FlexColumn,
  FlexRow,
  FlexWrap,
  Inline,
  Block,
  InlineBlock,
  None,
}

impl Display for RespoDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoDisplay::Flex => write!(f, "flex"),
      RespoDisplay::FlexColumn => write!(f, "flex-column"),
      RespoDisplay::FlexRow => write!(f, "flex-row"),
      RespoDisplay::FlexWrap => write!(f, "flex-wrap"),
      RespoDisplay::Inline => write!(f, "inline"),
      RespoDisplay::Block => write!(f, "block"),
      RespoDisplay::InlineBlock => write!(f, "inline-block"),
      RespoDisplay::None => write!(f, "none"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoFlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
}

impl Display for RespoFlexWrap {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoFlexWrap::NoWrap => write!(f, "nowrap"),
      RespoFlexWrap::Wrap => write!(f, "wrap"),
      RespoFlexWrap::WrapReverse => write!(f, "wrap-reverse"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoFlexDirection {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl Display for RespoFlexDirection {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoFlexDirection::Row => write!(f, "row"),
      RespoFlexDirection::RowReverse => write!(f, "row-reverse"),
      RespoFlexDirection::Column => write!(f, "column"),
      RespoFlexDirection::ColumnReverse => write!(f, "column-reverse"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoVerticalAlign {
  Top,
  Middle,
  Bottom,
}

impl Display for RespoVerticalAlign {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoVerticalAlign::Top => write!(f, "top"),
      RespoVerticalAlign::Middle => write!(f, "middle"),
      RespoVerticalAlign::Bottom => write!(f, "bottom"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoFlexJustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
}

impl Display for RespoFlexJustifyContent {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoFlexJustifyContent::FlexStart => write!(f, "flex-start"),
      RespoFlexJustifyContent::FlexEnd => write!(f, "flex-end"),
      RespoFlexJustifyContent::Center => write!(f, "center"),
      RespoFlexJustifyContent::SpaceBetween => write!(f, "space-between"),
      RespoFlexJustifyContent::SpaceAround => write!(f, "space-around"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoFlexAlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}

impl Display for RespoFlexAlignItems {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoFlexAlignItems::FlexStart => write!(f, "flex-start"),
      RespoFlexAlignItems::FlexEnd => write!(f, "flex-end"),
      RespoFlexAlignItems::Center => write!(f, "center"),
      RespoFlexAlignItems::Baseline => write!(f, "baseline"),
      RespoFlexAlignItems::Stretch => write!(f, "stretch"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RespoBorderStyle {
  Solid,
  Dashed,
  Dotted,
}

impl Display for RespoBorderStyle {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoBorderStyle::Solid => write!(f, "solid"),
      RespoBorderStyle::Dashed => write!(f, "dashed"),
      RespoBorderStyle::Dotted => write!(f, "dotted"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RespoBackgroundSize {
  Cover,
  Contain,
  Wh(f32, f32),
}

impl Display for RespoBackgroundSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Cover => write!(f, "cover"),
      Self::Contain => write!(f, "contain"),
      Self::Wh(w, h) => write!(f, "{}px {}px", w, h),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RespoOverflow {
  Visible,
  Hidden,
  Scroll,
  Auto,
}

impl Display for RespoOverflow {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoOverflow::Visible => write!(f, "visible"),
      RespoOverflow::Hidden => write!(f, "hidden"),
      RespoOverflow::Scroll => write!(f, "scroll"),
      RespoOverflow::Auto => write!(f, "auto"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RespoTransform {
  Translate(f32, f32),
  Scale(f32, f32),
  Rotate(f32),
  Skew(f32, f32),
  Matrix(f32, f32, f32, f32, f32, f32),
}

impl Display for RespoTransform {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      RespoTransform::Translate(x, y) => write!(f, "translate({}px, {}px)", x, y),
      RespoTransform::Scale(x, y) => write!(f, "scale({}, {})", x, y),
      RespoTransform::Rotate(r) => write!(f, "rotate({}deg)", r),
      RespoTransform::Skew(x, y) => write!(f, "skew({}deg, {}deg)", x, y),
      RespoTransform::Matrix(a, b, c, d, e, g) => write!(f, "matrix({a}, {b}, {c}, {d}, {e}, {g})"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RespoTimingFunction {
  Linear,
  Ease,
  EaseIn,
  EaseOut,
  EaseInOut,
  StepStart,
  StepEnd,
}

impl Display for RespoTimingFunction {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      RespoTimingFunction::Linear => write!(f, "linear"),
      RespoTimingFunction::Ease => write!(f, "ease"),
      RespoTimingFunction::EaseIn => write!(f, "ease-in"),
      RespoTimingFunction::EaseOut => write!(f, "ease-out"),
      RespoTimingFunction::EaseInOut => write!(f, "ease-in-out"),
      RespoTimingFunction::StepStart => write!(f, "step-start"),
      RespoTimingFunction::StepEnd => write!(f, "step-end"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RespoTextOverflow {
  Clip,
  Ellipsis,
}

impl Display for RespoTextOverflow {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      RespoTextOverflow::Clip => write!(f, "clip"),
      RespoTextOverflow::Ellipsis => write!(f, "ellipsis"),
    }
  }
}
