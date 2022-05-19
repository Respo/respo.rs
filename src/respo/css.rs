use std::{
  collections::{HashMap, HashSet},
  fmt::{self, format, Display, Formatter},
  sync::RwLock,
};

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};

lazy_static::lazy_static! {
  static ref CLASS_NAME_IN_TAGS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

/// it provides ADT interfaces as APIs, but internally it's maintained dynamically.
/// it's easily diffed in a hashmap.
/// and when it's sent to JS APIs, it's still in strings, which is also dynamic.
/// TODO order of rules might matter in edge cases
#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct RespoStyle(pub HashMap<String, String>);

impl RespoStyle {
  pub fn add(&mut self, rule: CssRule) -> &mut Self {
    let (property, value) = rule.get_pair();
    self.0.insert(property, value);
    self
  }
  pub fn insert(&mut self, property: String, value: String) -> &mut Self {
    self.0.insert(property, value);
    self
  }

  pub fn render_rules(rules: &[(String, Self)]) -> String {
    let mut result = String::new();
    for rule in rules {
      let (query, value) = rule;
      result.push_str(&format!("{} {{\n{}\n}}", query, value));
    }
    result
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
pub enum CssRule {
  // box styles
  Width(CssSize),
  Height(CssSize),
  Margin(f32),
  Margin4(f32, f32, f32, f32),
  Padding(f32),
  Padding4(f32, f32, f32, f32),
  Border(f32, CssBorderStyle, CssColor),
  Outline(f32, CssBorderStyle, CssColor),
  BoxShadow(f32, f32, f32, f32, CssColor),
  BorderRadius(f32),
  Overflow(CssOverflow),
  MaxWidth(CssSize),
  MaxHeight(CssSize),
  Opacity(f32),
  // background
  BackgroundColor(CssColor),
  BackgroundImage(String),
  BackgroundSize(CssBackgroundSize),
  BackgroundFilter(String),
  // text styles
  Color(CssColor),
  FontFamily(String),
  FontSize(f32),
  FontStyle(CssFontStyle),
  TextShadow(f32, f32, f32, CssColor),
  LineHeight(CssLineHeight),
  VerticalAlign(CssVerticalAlign),
  TextOverflow(CssTextOverflow),
  Cursor(String),
  // flex styles
  Display(CssDisplay),
  FlexDirection(CssFlexDirection),
  FlexWrap(CssFlexWrap),
  JustifyContent(CssFlexJustifyContent),
  AlignItems(CssFlexAlignItems),
  // positions
  Position(RespoPosition),
  Top(CssSize),
  Left(CssSize),
  Right(CssSize),
  Bottom(CssSize),
  ZIndex(i32),
  // transform
  Transform(CssTransform),
  TransitionDuration(f32),
  TransitionProperty(String),
  TransitionTimingFunction(CssTimingFunction),
  TransitionDelay(f32),
}

impl Display for CssRule {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let pair = self.get_pair();
    f.write_str(&pair.0)?;
    f.write_str(": ")?;
    f.write_str(&pair.1)?;
    f.write_str("; ")
  }
}

impl CssRule {
  pub fn get_pair(&self) -> (String, String) {
    match self {
      Self::Width(size) => ("width".to_owned(), size.to_string()),
      Self::Height(size) => ("height".to_owned(), size.to_string()),
      Self::Margin(margin) => ("margin".to_owned(), format!("{margin}px")),
      Self::Margin4(top, right, bottom, left) => ("margin".to_owned(), format!("{top}px {right}px {bottom}px {left}px")),
      Self::Padding(padding) => ("padding".to_owned(), format!("{padding}px")),
      Self::Padding4(top, right, bottom, left) => ("padding".to_owned(), format!("{top}px {right}px {bottom}px {left}px")),
      Self::Border(width, style, color) => ("border".to_owned(), format!("{width}px {style} {color}")),
      Self::Outline(width, style, color) => ("outline".to_owned(), format!("{width}px {style} {color}")),
      Self::BoxShadow(x, y, blur, spread, color) => ("box-shadow".to_owned(), format!("{x}px {y}px {blur}px {spread}px {color}")),
      Self::BorderRadius(radius) => ("border-radius".to_owned(), format!("{radius}px")),
      Self::Overflow(overflow) => ("overflow".to_owned(), overflow.to_string()),
      Self::MaxWidth(size) => ("max-width".to_owned(), format!("{size}px")),
      Self::MaxHeight(size) => ("max-height".to_owned(), format!("{size}px")),
      Self::Opacity(opacity) => ("opacity".to_owned(), opacity.to_string()),
      Self::BackgroundColor(color) => ("background-color".to_owned(), color.to_string()),
      Self::BackgroundImage(image) => ("background-image".to_owned(), image.to_string()),
      Self::BackgroundSize(size) => ("background-size".to_owned(), size.to_string()),
      Self::BackgroundFilter(filter) => ("background-filter".to_owned(), filter.to_string()),
      Self::Color(color) => ("color".to_owned(), color.to_string()),
      Self::FontFamily(family) => ("font-family".to_owned(), family.to_string()),
      Self::FontSize(size) => ("font-size".to_owned(), format!("{size}px")),
      Self::FontStyle(style) => ("font-style".to_owned(), style.to_string()),
      Self::TextShadow(x, y, blur, color) => ("text-shadow".to_owned(), format!("{x}px {y}px {blur}px {color}px")),
      Self::LineHeight(line_height) => ("line-height".to_owned(), line_height.to_string()),
      Self::VerticalAlign(align) => ("vertical-align".to_owned(), align.to_string()),
      Self::TextOverflow(overflow) => ("text-overflow".to_owned(), overflow.to_string()),
      Self::Display(display) => ("display".to_owned(), display.to_string()),
      Self::FlexDirection(direction) => ("flex-direction".to_owned(), direction.to_string()),
      Self::FlexWrap(wrap) => ("flex-wrap".to_owned(), wrap.to_string()),
      Self::JustifyContent(content) => ("justify-content".to_owned(), content.to_string()),
      Self::AlignItems(align) => ("align-items".to_owned(), align.to_string()),
      Self::Position(position) => ("position".to_owned(), position.to_string()),
      Self::Top(size) => ("top".to_owned(), size.to_string()),
      Self::Left(size) => ("left".to_owned(), size.to_string()),
      Self::Right(size) => ("right".to_owned(), size.to_string()),
      Self::Bottom(size) => ("bottom".to_owned(), size.to_string()),
      Self::ZIndex(index) => ("z-index".to_owned(), index.to_string()),
      Self::Transform(transform) => ("transform".to_owned(), transform.to_string()),
      Self::TransitionDuration(duration) => ("transition-duration".to_owned(), duration.to_string()),
      Self::TransitionProperty(property) => ("transition-property".to_owned(), property.to_string()),
      Self::TransitionTimingFunction(timing_function) => ("transition-timing-function".to_owned(), timing_function.to_string()),
      Self::TransitionDelay(delay) => ("transition-delay".to_owned(), delay.to_string()),
      Self::Cursor(v) => ("cursor".to_owned(), v.to_string()),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CssSize {
  Auto,
  Px(f32),
  Percent(f32),
  Vw(f32),
  Wh(f32),
  /// may be calc or something
  Custom(String),
}

impl Display for CssSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Auto => write!(f, "auto"),
      Self::Px(v) => write!(f, "{}px", v),
      Self::Percent(v) => write!(f, "{}%", v),
      Self::Vw(v) => write!(f, "{}vw", v),
      Self::Wh(v) => write!(f, "{}wh", v),
      Self::Custom(v) => write!(f, "{}", v),
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
        Self::Static => "static",
        Self::Relative => "relative",
        Self::Absolute => "absolute",
        Self::Fixed => "fixed",
      }
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssColor {
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

impl Display for CssColor {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Hsla(h, s, l, a) => format!("hsla({}, {}%, {}%, {})", h, s, l, a),
        Self::Hcla(h, c, l, a) => format!("hcla({}, {}, {}, {})", h, c, l, a), // TODO
        Self::Rgba(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
        Self::Hex(h) => format!("#{:02x}", h),
        Self::Red => "red".to_string(),
        Self::Green => "green".to_string(),
        Self::Blue => "blue".to_string(),
        Self::White => "white".to_string(),
        Self::Black => "black".to_string(),
        Self::Gray => "gray".to_string(),
        Self::Yellow => "yellow".to_string(),
        Self::Purple => "purple".to_string(),
        Self::Cyan => "cyan".to_string(),
        Self::Orange => "orange".to_string(),
        Self::Pink => "pink".to_string(),
      }
    )
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssLineHeight {
  Em(f32),
  Px(f32),
}

impl Display for CssLineHeight {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      CssLineHeight::Em(v) => write!(f, "{}em", v),
      CssLineHeight::Px(v) => write!(f, "{}px", v),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFontStyle {
  Normal,
  Italic,
  Oblique,
}

impl Display for CssFontStyle {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Normal => write!(f, "normal"),
      Self::Italic => write!(f, "italic"),
      Self::Oblique => write!(f, "oblique"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssDisplay {
  Flex,
  FlexColumn,
  FlexRow,
  FlexWrap,
  Inline,
  Block,
  InlineBlock,
  None,
}

impl Display for CssDisplay {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Flex => write!(f, "flex"),
      Self::FlexColumn => write!(f, "flex-column"),
      Self::FlexRow => write!(f, "flex-row"),
      Self::FlexWrap => write!(f, "flex-wrap"),
      Self::Inline => write!(f, "inline"),
      Self::Block => write!(f, "block"),
      Self::InlineBlock => write!(f, "inline-block"),
      Self::None => write!(f, "none"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
}

impl Display for CssFlexWrap {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::NoWrap => write!(f, "nowrap"),
      Self::Wrap => write!(f, "wrap"),
      Self::WrapReverse => write!(f, "wrap-reverse"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexDirection {
  Row,
  RowReverse,
  Column,
  ColumnReverse,
}

impl Display for CssFlexDirection {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Row => write!(f, "row"),
      Self::RowReverse => write!(f, "row-reverse"),
      Self::Column => write!(f, "column"),
      Self::ColumnReverse => write!(f, "column-reverse"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssVerticalAlign {
  Top,
  Middle,
  Bottom,
}

impl Display for CssVerticalAlign {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Top => write!(f, "top"),
      Self::Middle => write!(f, "middle"),
      Self::Bottom => write!(f, "bottom"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexJustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
}

impl Display for CssFlexJustifyContent {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::FlexStart => write!(f, "flex-start"),
      Self::FlexEnd => write!(f, "flex-end"),
      Self::Center => write!(f, "center"),
      Self::SpaceBetween => write!(f, "space-between"),
      Self::SpaceAround => write!(f, "space-around"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFlexAlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Baseline,
  Stretch,
}

impl Display for CssFlexAlignItems {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::FlexStart => write!(f, "flex-start"),
      Self::FlexEnd => write!(f, "flex-end"),
      Self::Center => write!(f, "center"),
      Self::Baseline => write!(f, "baseline"),
      Self::Stretch => write!(f, "stretch"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssBorderStyle {
  Solid,
  Dashed,
  Dotted,
}

impl Display for CssBorderStyle {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Solid => write!(f, "solid"),
      Self::Dashed => write!(f, "dashed"),
      Self::Dotted => write!(f, "dotted"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CssBackgroundSize {
  Cover,
  Contain,
  Wh(f32, f32),
}

impl Display for CssBackgroundSize {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Cover => write!(f, "cover"),
      Self::Contain => write!(f, "contain"),
      Self::Wh(w, h) => write!(f, "{}px {}px", w, h),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssOverflow {
  Visible,
  Hidden,
  Scroll,
  Auto,
}

impl Display for CssOverflow {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Visible => write!(f, "visible"),
      Self::Hidden => write!(f, "hidden"),
      Self::Scroll => write!(f, "scroll"),
      Self::Auto => write!(f, "auto"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CssTransform {
  Translate(f32, f32),
  Scale(f32, f32),
  Rotate(f32),
  Skew(f32, f32),
  Matrix(f32, f32, f32, f32, f32, f32),
}

impl Display for CssTransform {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::Translate(x, y) => write!(f, "translate({}px, {}px)", x, y),
      Self::Scale(x, y) => write!(f, "scale({}, {})", x, y),
      Self::Rotate(r) => write!(f, "rotate({}deg)", r),
      Self::Skew(x, y) => write!(f, "skew({}deg, {}deg)", x, y),
      Self::Matrix(a, b, c, d, e, g) => write!(f, "matrix({a}, {b}, {c}, {d}, {e}, {g})"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CssTimingFunction {
  Linear,
  Ease,
  EaseIn,
  EaseOut,
  EaseInOut,
  StepStart,
  StepEnd,
}

impl Display for CssTimingFunction {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Linear => write!(f, "linear"),
      Self::Ease => write!(f, "ease"),
      Self::EaseIn => write!(f, "ease-in"),
      Self::EaseOut => write!(f, "ease-out"),
      Self::EaseInOut => write!(f, "ease-in-out"),
      Self::StepStart => write!(f, "step-start"),
      Self::StepEnd => write!(f, "step-end"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssTextOverflow {
  Clip,
  Ellipsis,
}

impl Display for CssTextOverflow {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Self::Clip => write!(f, "clip"),
      Self::Ellipsis => write!(f, "ellipsis"),
    }
  }
}

/// inserts CSS as `<style .. />` under `<head ... />` element
/// notice that the code only generats once and being cached as DOM states,
/// NOT working for dynamic styles that changes over time, use inline styles instead.
pub fn declare_static_style<T>(name: T, rules: &[(String, &[CssRule])]) -> String
where
  T: Into<String> + Clone,
{
  let defined_styles = CLASS_NAME_IN_TAGS.write().expect("access styles");
  if defined_styles.contains(&name.to_owned().into()) {
    name.into()
  } else {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("load document");
    let head = document.head().expect("head");
    let style_tag = document.create_element("style").expect("create style tag");

    let mut styles = String::from("");
    for (query, properties) in rules {
      styles.push_str(&query.replace("$0", &format!(".{}", &name.to_owned().into())));
      styles.push_str(" {\n");
      for p in *properties {
        styles.push_str(&p.to_string());
        styles.push('\n');
      }
      styles.push_str("}\n");
    }

    style_tag.dyn_ref::<Element>().expect("into element").set_inner_html(&styles);
    head
      .append_child(style_tag.dyn_ref::<Element>().expect("get element"))
      .expect("add style");

    name.into()
  }
}
