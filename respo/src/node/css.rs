//! Define CSS styles in Rust and generate CSS class-name.
//!
//! ```rust
//! use respo::css::*;
//! respo::static_styles!(
//!  style_done_button,
//!  (
//!    "&",
//!    respo_style()
//!      .width(24.px())
//!      .height(24.px())
//!      .margin(4.)
//!      .cursor("pointer")
//!      .background_color(CssColor::Hsl(20, 90, 70)),
//!  )
//! );
//! ```
//!
//! then `style_done_button()` returns the class name, while CSS is generated and injected into the `<style/>`.

mod css_size;

use std::{
  collections::HashSet,
  fmt::{self, Display, Formatter, Write},
  rc::Rc,
  sync::RwLock,
};

use css_size::CssPx;
use hsluv::hsluv_to_rgb;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub use css_size::{ConvertRespoCssSize, CssSize};

lazy_static::lazy_static! {
  static ref CLASS_NAME_IN_TAGS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

/// it provides ADT interfaces as APIs, but internally it's maintained dynamically.
/// it's easily diffed in a hashmap.
/// and when it's sent to JS APIs, it's still in strings, which is also dynamic.
/// TODO order of rules might matter in edge cases
#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct RespoStyle(pub Vec<(Rc<str>, String)>);

/// this is an alias
pub fn respo_style() -> RespoStyle {
  RespoStyle::default()
}

impl RespoStyle {
  pub fn insert(self, property: &str, value: String) -> Self {
    let mut xs = self.0;
    xs.push((Rc::from(property), value));
    Self(xs)
  }

  pub fn render_rules(rules: &[(String, Self)]) -> String {
    let mut result = String::new();
    for rule in rules {
      let (query, value) = rule;
      let _ = write!(result, "{} {{\n{}\n}}", query, value);
    }
    result
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}

impl Display for RespoStyle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (key, value) in self.0.iter() {
      writeln!(f, "{}: {};", key, value)?;
    }
    Ok(())
  }
}

impl RespoStyle {
  pub fn width(self, rule: CssSize) -> Self {
    self.insert("width", rule.to_string())
  }
  pub fn height(self, rule: CssSize) -> Self {
    self.insert("height", rule.to_string())
  }
  pub fn margin<T: Into<CssPx>>(self, m: T) -> Self {
    self.insert("margin", format!("{}", m.into()))
  }
  pub fn margin4<T: Into<CssPx>>(self, top: T, right: T, bottom: T, left: T) -> Self {
    self.insert(
      "margin",
      format!("{} {} {} {}", top.into(), right.into(), bottom.into(), left.into()),
    )
  }
  pub fn padding<T: Into<CssPx>>(self, p: T) -> Self {
    self.insert("padding", format!("{}", p.into()))
  }
  pub fn padding4<T: Into<CssPx>>(self, top: T, right: T, bottom: T, left: T) -> Self {
    self.insert(
      "padding",
      format!("{} {} {} {}", top.into(), right.into(), bottom.into(), left.into()),
    )
  }
  pub fn border(self, rule: Option<(f32, CssBorderStyle, CssColor)>) -> Self {
    match rule {
      Some((width, style, color)) => self
        .insert("border-width", format!("{}px", width))
        .insert("border-style", format!("{}", style))
        .insert("border-color", format!("{}", color)),
      None => self
        .insert("border-width", "0px".to_owned())
        .insert("border-style", "none".to_owned())
        .insert("border-color", "transparent".to_owned()),
    }
  }
  pub fn outline(self, rule: Option<(f32, CssBorderStyle, CssColor)>) -> Self {
    match rule {
      Some((width, style, color)) => self
        .insert("outline-width", format!("{}px", width))
        .insert("outline-style", format!("{}", style))
        .insert("outline-color", format!("{}", color)),
      None => self
        .insert("outline-width", "0px".to_owned())
        .insert("outline-style", "none".to_owned())
        .insert("outline-color", "transparent".to_owned()),
    }
  }
  pub fn box_shadow(self, x: f32, y: f32, blur: f32, spread: f32, color: CssColor) -> Self {
    self.insert("box-shadow", format!("{}px {}px {}px {}px {}", x, y, blur, spread, color))
  }
  pub fn border_radius(self, r: f32) -> Self {
    self.insert("border-radius", format!("{}px", r))
  }
  pub fn border_color(self, color: CssColor) -> Self {
    self.insert("border-color", color.to_string())
  }
  pub fn overflow(self, rule: CssOverflow) -> Self {
    self.insert("overflow", rule.to_string())
  }
  pub fn max_width(self, rule: CssSize) -> Self {
    self.insert("max-width", rule.to_string())
  }
  pub fn max_height(self, rule: CssSize) -> Self {
    self.insert("max-height", rule.to_string())
  }
  pub fn min_width(self, rule: CssSize) -> Self {
    self.insert("min-width", rule.to_string())
  }
  pub fn min_height(self, rule: CssSize) -> Self {
    self.insert("min-height", rule.to_string())
  }
  pub fn opacity(self, o: f32) -> Self {
    self.insert("opacity", format!("{}", o))
  }
  pub fn background_color(self, color: CssColor) -> Self {
    self.insert("background-color", color.to_string())
  }
  pub fn background_image(self, image: String) -> Self {
    self.insert("background-image", image)
  }
  pub fn background_size(self, size: CssBackgroundSize) -> Self {
    self.insert("background-size", size.to_string())
  }
  pub fn background_filter(self, f: String) -> Self {
    self.insert("background-filter", f)
  }
  pub fn color(self, color: CssColor) -> Self {
    self.insert("color", color.to_string())
  }
  pub fn font_family(self, font: String) -> Self {
    self.insert("font-family", font)
  }
  pub fn font_size(self, size: f32) -> Self {
    self.insert("font-size", format!("{}px", size))
  }
  pub fn font_style(self, style: CssFontStyle) -> Self {
    self.insert("font-style", style.to_string())
  }
  pub fn font_weight(self, weight: CssFontWeight) -> Self {
    self.insert("font-weight", weight.to_string())
  }
  pub fn text_shadow(self, x: f32, y: f32, blur: f32, color: CssColor) -> Self {
    self.insert("text-shadow", format!("{}px {}px {}px {}", x, y, blur, color))
  }
  pub fn line_height(self, height: CssLineHeight) -> Self {
    self.insert("line-height", height.to_string())
  }
  pub fn text_align(self, align: CssTextAlign) -> Self {
    self.insert("text-align", align.to_string())
  }
  pub fn vertical_align(self, align: CssVerticalAlign) -> Self {
    self.insert("vertical-align", align.to_string())
  }
  pub fn text_overflow(self, overflow: CssTextOverflow) -> Self {
    self.insert("text-overflow", overflow.to_string())
  }
  pub fn cursor(self, cursor: &str) -> Self {
    self.insert("cursor", cursor.to_owned())
  }
  pub fn display(self, display: CssDisplay) -> Self {
    self.insert("display", display.to_string())
  }
  pub fn flex_direction(self, direction: CssFlexDirection) -> Self {
    self.insert("flex-direction", direction.to_string())
  }
  pub fn flex_wrap(self, wrap: CssFlexWrap) -> Self {
    self.insert("flex-wrap", wrap.to_string())
  }
  pub fn justify_content(self, content: CssFlexJustifyContent) -> Self {
    self.insert("justify-content", content.to_string())
  }
  pub fn align_items(self, items: CssFlexAlignItems) -> Self {
    self.insert("align-items", items.to_string())
  }
  pub fn position(self, position: CssPosition) -> Self {
    self.insert("position", position.to_string())
  }
  pub fn top(self, top: CssSize) -> Self {
    self.insert("top", top.to_string())
  }
  pub fn left(self, left: CssSize) -> Self {
    self.insert("left", left.to_string())
  }
  pub fn right(self, right: CssSize) -> Self {
    self.insert("right", right.to_string())
  }
  pub fn bottom(self, bottom: CssSize) -> Self {
    self.insert("bottom", bottom.to_string())
  }
  pub fn z_index(self, z_index: i32) -> Self {
    self.insert("z-index", z_index.to_string())
  }
  pub fn transform(self, transform: CssTransform) -> Self {
    self.insert("transform", transform.to_string())
  }
  pub fn transform_origin(self, origin: String) -> Self {
    self.insert("tranform-origin", origin)
  }
  pub fn transition_duration(self, duration: f32) -> Self {
    self.insert("transition-duration", format!("{}ms", duration))
  }
  pub fn transition_delay(self, delay: f32) -> Self {
    self.insert("transition-delay", format!("{}ms", delay))
  }
  pub fn transform_timing_function(self, function: CssTimingFunction) -> Self {
    self.insert("transition-timing-function", function.to_string())
  }
  pub fn transform_property(self, property: String) -> Self {
    self.insert("transition-property", property)
  }
  pub fn box_sizing(self, box_sizing: CssBoxSizing) -> Self {
    self.insert("box-sizing", box_sizing.to_string())
  }
  pub fn text_decoration(self, decoration: CssTextDecoration) -> Self {
    self.insert("text-decoration", decoration.to_string())
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssPosition {
  Static,
  Relative,
  Absolute,
  Fixed,
}

impl Display for CssPosition {
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
  /// 0~360, 0~100, 0~100, 0~1
  Hsla(f32, f32, f32, f32),
  /// 0~360, 0~100, 0~100
  Hsl(u16, u16, u16),
  /// 0~360, 0~100, 0~100, 0~1
  Hsluva(f32, f32, f32, f32),
  /// 0~360, 0~100, 0~100
  Hsluv(u16, u16, u16),
  /// 0~255, 0~255, 0~255, 0~1
  Rgba(u16, u16, u16, f32),
  /// 0~255, 0~255, 0~255
  Rgb(u16, u16, u16),
  /// 0~255, 0~255, 0~255
  Hex(u16, u16, u16),
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
        Self::Hsl(h, s, l) => format!("hsl({}, {}%, {}%)", h, s, l),
        Self::Hsluva(h, c, l, a) => {
          let (r, g, b) = hsluv_to_rgb((*h as f64, *c as f64, *l as f64));
          format!("rgba({}, {}, {}, {})", r * 256., g * 256., b * 256., a)
        }
        Self::Hsluv(h, c, l) => {
          let (r, g, b) = hsluv_to_rgb((*h as f64, *c as f64, *l as f64));
          format!("rgb({}, {}, {})", r * 256., g * 256., b * 256.)
        }
        Self::Rgba(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
        Self::Rgb(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
        Self::Hex(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b),
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
  SpaceEvenly,
}

impl Display for CssFlexJustifyContent {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Self::FlexStart => write!(f, "flex-start"),
      Self::FlexEnd => write!(f, "flex-end"),
      Self::Center => write!(f, "center"),
      Self::SpaceBetween => write!(f, "space-between"),
      Self::SpaceAround => write!(f, "space-around"),
      Self::SpaceEvenly => write!(f, "space-evenly"),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssBoxSizing {
  ContentBox,
  BorderBox,
}

impl Display for CssBoxSizing {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Self::ContentBox => write!(f, "content-box"),
      Self::BorderBox => write!(f, "border-box"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssTextAlign {
  Left,
  Right,
  Center,
  Justify,
}

impl Display for CssTextAlign {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Self::Left => write!(f, "left"),
      Self::Right => write!(f, "right"),
      Self::Center => write!(f, "center"),
      Self::Justify => write!(f, "justify"),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CssFontWeight {
  Normal,
  Bold,
  Bolder,
  Lighter,
  Weight(u32),
}

impl Display for CssFontWeight {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Self::Normal => write!(f, "normal"),
      Self::Bold => write!(f, "bold"),
      Self::Bolder => write!(f, "bolder"),
      Self::Lighter => write!(f, "lighter"),
      Self::Weight(w) => write!(f, "{}", w),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CssTextDecoration {
  Underline,
  None,
}

impl Display for CssTextDecoration {
  fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
    match self {
      Self::Underline => write!(f, "underline"),
      Self::None => write!(f, "none"),
    }
  }
}

/// does internal work inside the macro `static_style!(name, &styles)`.
/// inserts CSS as `<style .. />` under `<head ... />` element
/// notice that the code only generats once and being cached as DOM states,
///
/// NOT working for dynamic styles that changes over time, use inline styles instead.
pub fn declare_static_style<T, U>(name: T, rules: &[(U, RespoStyle)]) -> String
where
  T: Into<String> + Clone,
  U: Into<String> + Clone + Display,
{
  let mut defined_styles = CLASS_NAME_IN_TAGS.write().expect("access styles");
  if defined_styles.contains(&name.to_owned().into()) {
    name.into()
  } else {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("load document");
    let head = document.head().expect("head");
    let style_tag = document.create_element("style").expect("create style tag");
    style_tag
      .set_attribute("id", &format!("def__{}", name.to_owned().into()))
      .expect("name tag");

    let mut styles = String::from("");
    for (query, properties) in rules {
      styles.push_str(
        &query
          .to_string()
          .replace("$0", &format!(".{}", &name.to_owned().into()))
          .replace('&', &format!(".{}", &name.to_owned().into())),
      );
      styles.push_str(" {\n");
      styles.push_str(&properties.to_string());
      styles.push_str("}\n");
    }

    style_tag.dyn_ref::<Element>().expect("into element").set_inner_html(&styles);
    head
      .append_child(style_tag.dyn_ref::<Element>().expect("get element"))
      .expect("add style");

    defined_styles.insert(name.to_owned().into());

    name.into()
  }
}
/// does internal work inside the macro `contained_style!(name, &styles)`.
/// inserts CSS as `<style .. />` under `<head ... />` element
/// notice that the code only generats once and being cached as DOM states, with extra `<contained> { ... }` wrapper
///
/// NOT working for dynamic styles that changes over time, use inline styles instead.
pub fn declare_contained_style<T, U>(name: T, rules: &[(Option<String>, U, RespoStyle)]) -> String
where
  T: Into<String> + Clone,
  U: Into<String> + Clone + Display,
{
  let mut defined_styles = CLASS_NAME_IN_TAGS.write().expect("access styles");
  if defined_styles.contains(&name.to_owned().into()) {
    name.into()
  } else {
    let window = web_sys::window().expect("window");
    let document = window.document().expect("load document");
    let head = document.head().expect("head");
    let style_tag = document.create_element("style").expect("create style tag");
    style_tag
      .set_attribute("id", &format!("def__{}", name.to_owned().into()))
      .expect("name tag");

    let mut styles = String::from("");
    for (contained, query, properties) in rules {
      styles.push_str(
        &query
          .to_string()
          .replace("$0", &format!(".{}", &name.to_owned().into()))
          .replace('&', &format!(".{}", &name.to_owned().into())),
      );
      styles.push_str(" {\n");
      styles.push_str(&properties.to_string());
      styles.push_str("}\n");

      if let Some(contained) = contained {
        styles = format!("{} {{\n{}\n}}", contained, styles);
      }
    }

    style_tag.dyn_ref::<Element>().expect("into element").set_inner_html(&styles);
    head
      .append_child(style_tag.dyn_ref::<Element>().expect("get element"))
      .expect("add style");

    defined_styles.insert(name.to_owned().into());

    name.into()
  }
}

/// turns `src/a/b.rs` into `a_b`, (used inside macro)
pub fn css_name_from_path(p: &str) -> String {
  let mut s = p.to_owned();
  if let Some(x) = s.strip_prefix("src/") {
    s = x.to_string();
  }
  if let Some(x) = s.strip_suffix(".rs") {
    s = x.to_string();
  }
  s.replace("::", "_").replace(['/', '.'], "_")
}

/// macro to create a public function of CSS rules with a slice at current file scope,
/// ```rust
/// respo::static_style_seq!(the_name,
///   &[
///     ("&", respo::css::respo_style())
///   ]
/// );
/// ```
/// gets a function like:
/// ```ignore
/// pub fn the_name() -> String
/// ```
#[macro_export]
macro_rules! static_style_seq {
  ($a:ident, $b:expr) => {
    pub fn $a() -> String {
      // let name = $crate::css_name_from_path(std::file!());
      let name = $crate::css::css_name_from_path(std::module_path!());
      $crate::css::declare_static_style(format!("{}__{}", name, stringify!($a)), $b)
    }
  };
}

/// macro to create a public function of CSS rules(up to 5 tuples) at current file scope,
/// ```rust
/// respo::static_styles!(the_name,
///   ("&", respo::css::respo_style())
/// );
/// ```
/// gets a function like:
/// ```ignore
/// pub fn the_name() -> String
/// ```
/// if you have more styles to specify, use `static_style_seq!` instead.
#[macro_export]
macro_rules! static_styles {
  ($a:ident, $($b:expr),* $(,)?) => {
    $crate::static_style_seq!($a, &[$($b),*]);
  };
}

/// macro to create a public function of CSS rules with a slice at current file scope,
/// ```rust
/// respo::contained_style_seq!(the_name,
///   &[
///     (Some("@container".to_string()), "&", respo::css::respo_style())
///   ]
/// );
/// ```
/// gets a function like:
/// ```ignore
/// pub fn the_name() -> String
/// ```
#[macro_export]
macro_rules! contained_style_seq {
  ($a:ident, $b:expr) => {
    pub fn $a() -> String {
      // let name = $crate::css_name_from_path(std::file!());
      let name = $crate::css::css_name_from_path(std::module_path!());
      $crate::css::declare_contained_style(format!("{}__{}", name, stringify!($a)), $b)
    }
  };
}

/// macro to create a public function of CSS rules(up to 5 tuples) at current file scope,
/// ```rust
/// // Example with multiple container queries
/// use respo::css::{CssSize, CssColor};
/// respo::contained_style_seq!(responsive_card,
///   &[
///     (None, "&", respo::css::respo_style().width(CssSize::Px(200.))),
///     (Some("@container (min-width: 300px)".to_string()), "&", respo::css::respo_style().width(CssSize::Px(300.))),
///   ]
/// );
///
/// // Example combining with media queries
/// respo::contained_style_seq!(hybrid_responsive,
///   &[
///     (Some("@media (prefers-color-scheme: dark)".to_string()), "&",
///       respo::css::respo_style().background_color(CssColor::Rgb(20, 20, 20)))
///   ]
/// );
/// ```
/// if you have more styles to specify, use `contained_style_seq!` instead.
#[macro_export]
macro_rules! contained_styles {
  ($a:ident, $($b:expr),* $(,)?) => {
    $crate::contained_style_seq!($a, &[$($b),*]);
  };
}
