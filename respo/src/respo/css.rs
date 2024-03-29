use std::{
  collections::HashSet,
  fmt::Write,
  fmt::{self, Display, Formatter},
  sync::RwLock,
};

use hsluv::hsluv_to_rgb;
use wasm_bindgen::JsCast;
use web_sys::Element;

lazy_static::lazy_static! {
  static ref CLASS_NAME_IN_TAGS: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

/// it provides ADT interfaces as APIs, but internally it's maintained dynamically.
/// it's easily diffed in a hashmap.
/// and when it's sent to JS APIs, it's still in strings, which is also dynamic.
/// TODO order of rules might matter in edge cases
#[derive(Debug, Clone, PartialEq, Default, Eq)]
pub struct RespoStyle(pub Vec<(String, String)>);

impl RespoStyle {
  pub fn insert(&mut self, property: &str, value: String) -> &mut Self {
    self.0.push((property.to_owned(), value));
    self
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
  pub fn width(&mut self, rule: CssSize) -> &mut Self {
    self.insert("width", rule.to_string());
    self
  }
  pub fn height(&mut self, rule: CssSize) -> &mut Self {
    self.insert("height", rule.to_string());
    self
  }
  pub fn margin(&mut self, m: f32) -> &mut Self {
    self.insert("margin", format!("{}px", m));
    self
  }
  pub fn margin4(&mut self, top: f32, right: f32, bottom: f32, left: f32) -> &mut Self {
    self.insert("margin", format!("{}px {}px {}px {}px", top, right, bottom, left));
    self
  }
  pub fn padding(&mut self, p: f32) -> &mut Self {
    self.insert("padding", format!("{}px", p));
    self
  }
  pub fn padding4(&mut self, top: f32, right: f32, bottom: f32, left: f32) -> &mut Self {
    self.insert("padding", format!("{}px {}px {}px {}px", top, right, bottom, left));
    self
  }
  pub fn border(&mut self, rule: Option<(f32, CssBorderStyle, CssColor)>) -> &mut Self {
    match rule {
      Some((width, style, color)) => {
        self.insert("border-width", format!("{}px", width));
        self.insert("border-style", format!("{}", style));
        self.insert("border-color", format!("{}", color));
      }
      None => {
        self.insert("border-width", "0px".to_owned());
        self.insert("border-style", "none".to_owned());
        self.insert("border-color", "transparent".to_owned());
      }
    }
    self
  }
  pub fn outline(&mut self, rule: Option<(f32, CssBorderStyle, CssColor)>) -> &mut Self {
    match rule {
      Some((width, style, color)) => {
        self.insert("outline-width", format!("{}px", width));
        self.insert("outline-style", format!("{}", style));
        self.insert("outline-color", format!("{}", color));
      }
      None => {
        self.insert("outline-width", "0px".to_owned());
        self.insert("outline-style", "none".to_owned());
        self.insert("outline-color", "transparent".to_owned());
      }
    }
    self
  }
  pub fn box_shadow(&mut self, x: f32, y: f32, blur: f32, spread: f32, color: CssColor) -> &mut Self {
    self.insert("box-shadow", format!("{}px {}px {}px {}px {}", x, y, blur, spread, color));
    self
  }
  pub fn border_radius(&mut self, r: f32) -> &mut Self {
    self.insert("border-radius", format!("{}px", r));
    self
  }
  pub fn border_color(&mut self, color: CssColor) -> &mut Self {
    self.insert("border-color", color.to_string());
    self
  }
  pub fn overflow(&mut self, rule: CssOverflow) -> &mut Self {
    self.insert("overflow", rule.to_string());
    self
  }
  pub fn max_width(&mut self, rule: CssSize) -> &mut Self {
    self.insert("max-width", rule.to_string());
    self
  }
  pub fn max_height(&mut self, rule: CssSize) -> &mut Self {
    self.insert("max-height", rule.to_string());
    self
  }
  pub fn min_width(&mut self, rule: CssSize) -> &mut Self {
    self.insert("min-width", rule.to_string());
    self
  }
  pub fn min_height(&mut self, rule: CssSize) -> &mut Self {
    self.insert("min-height", rule.to_string());
    self
  }
  pub fn opacity(&mut self, o: f32) -> &mut Self {
    self.insert("opacity", format!("{}", o));
    self
  }
  pub fn background_color(&mut self, color: CssColor) -> &mut Self {
    self.insert("background-color", color.to_string());
    self
  }
  pub fn background_image(&mut self, image: String) -> &mut Self {
    self.insert("background-image", image);
    self
  }
  pub fn background_size(&mut self, size: CssBackgroundSize) -> &mut Self {
    self.insert("background-size", size.to_string());
    self
  }
  pub fn background_filter(&mut self, f: String) -> &mut Self {
    self.insert("background-filter", f);
    self
  }
  pub fn color(&mut self, color: CssColor) -> &mut Self {
    self.insert("color", color.to_string());
    self
  }
  pub fn font_family(&mut self, font: String) -> &mut Self {
    self.insert("font-family", font);
    self
  }
  pub fn font_size(&mut self, size: f32) -> &mut Self {
    self.insert("font-size", format!("{}px", size));
    self
  }
  pub fn font_style(&mut self, style: CssFontStyle) -> &mut Self {
    self.insert("font-style", style.to_string());
    self
  }
  pub fn font_weight(&mut self, weight: CssFontWeight) -> &mut Self {
    self.insert("font-weight", weight.to_string());
    self
  }
  pub fn text_shadow(&mut self, x: f32, y: f32, blur: f32, color: CssColor) -> &mut Self {
    self.insert("text-shadow", format!("{}px {}px {}px {}", x, y, blur, color));
    self
  }
  pub fn line_height(&mut self, height: CssLineHeight) -> &mut Self {
    self.insert("line-height", height.to_string());
    self
  }
  pub fn text_align(&mut self, align: CssTextAlign) -> &mut Self {
    self.insert("text-align", align.to_string());
    self
  }
  pub fn vertical_align(&mut self, align: CssVerticalAlign) -> &mut Self {
    self.insert("vertical-align", align.to_string());
    self
  }
  pub fn text_overflow(&mut self, overflow: CssTextOverflow) -> &mut Self {
    self.insert("text-overflow", overflow.to_string());
    self
  }
  pub fn cursor(&mut self, cursor: String) -> &mut Self {
    self.insert("cursor", cursor);
    self
  }
  pub fn display(&mut self, display: CssDisplay) -> &mut Self {
    self.insert("display", display.to_string());
    self
  }
  pub fn flex_direction(&mut self, direction: CssFlexDirection) -> &mut Self {
    self.insert("flex-direction", direction.to_string());
    self
  }
  pub fn flex_wrap(&mut self, wrap: CssFlexWrap) -> &mut Self {
    self.insert("flex-wrap", wrap.to_string());
    self
  }
  pub fn justify_content(&mut self, content: CssFlexJustifyContent) -> &mut Self {
    self.insert("justify-content", content.to_string());
    self
  }
  pub fn align_items(&mut self, items: CssFlexAlignItems) -> &mut Self {
    self.insert("align-items", items.to_string());
    self
  }
  pub fn position(&mut self, position: CssPosition) -> &mut Self {
    self.insert("position", position.to_string());
    self
  }
  pub fn top(&mut self, top: CssSize) -> &mut Self {
    self.insert("top", top.to_string());
    self
  }
  pub fn left(&mut self, left: CssSize) -> &mut Self {
    self.insert("left", left.to_string());
    self
  }
  pub fn right(&mut self, right: CssSize) -> &mut Self {
    self.insert("right", right.to_string());
    self
  }
  pub fn bottom(&mut self, bottom: CssSize) -> &mut Self {
    self.insert("bottom", bottom.to_string());
    self
  }
  pub fn z_index(&mut self, z_index: i32) -> &mut Self {
    self.insert("z-index", z_index.to_string());

    self
  }
  pub fn transform(&mut self, transform: CssTransform) -> &mut Self {
    self.insert("transform", transform.to_string());

    self
  }
  pub fn transform_origin(&mut self, origin: String) -> &mut Self {
    self.insert("tranform-origin", origin);
    self
  }
  pub fn transition_duration(&mut self, duration: f32) -> &mut Self {
    self.insert("transition-duration", format!("{}ms", duration));
    self
  }
  pub fn transition_delay(&mut self, delay: f32) -> &mut Self {
    self.insert("transition-delay", format!("{}ms", delay));
    self
  }
  pub fn transform_timing_function(&mut self, function: CssTimingFunction) -> &mut Self {
    self.insert("transition-timing-function", function.to_string());
    self
  }
  pub fn transform_property(&mut self, property: String) -> &mut Self {
    self.insert("transition-property", property);
    self
  }
  pub fn box_sizing(&mut self, box_sizing: CssBoxSizing) -> &mut Self {
    self.insert("box-sizing", box_sizing.to_string());
    self
  }
  pub fn text_decoration(&mut self, decoration: CssTextDecoration) -> &mut Self {
    self.insert("text-decoration", decoration.to_string());
    self
  }
}

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
pub fn declare_static_style<T, U>(name: T, rules: &[(U, &RespoStyle)]) -> String
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

/// turns `src/a/b.rs` into `a_b`, used inside macro
pub fn css_name_from_path(p: &str) -> String {
  let mut s = p.to_owned();
  if let Some(x) = s.strip_prefix("src/") {
    s = x.to_owned();
  }
  if let Some(x) = s.strip_suffix(".rs") {
    s = x.to_owned();
  }
  s = s.replace("::", "_");
  s = s.replace('/', "_");
  s = s.replace('.', "_");
  s
}

/// macro to create a public function of CSS rules with a slice at current file scope,
/// ```rust
/// respo::static_style_seq!(the_name,
///   &[
///     ("&", &respo::RespoStyle::default())
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
      let name = $crate::css_name_from_path(std::module_path!());
      $crate::declare_static_style(format!("{}__{}", name, stringify!($a)), $b)
    }
  };
}
pub use static_style_seq;

/// macro to create a public function of CSS rules(up to 5 tuples) at current file scope,
/// ```rust
/// respo::static_styles!(the_name,
///   ("&", &respo::RespoStyle::default())
/// );
/// ```
/// gets a function like:
/// ```ignore
/// pub fn the_name() -> String
/// ```
/// if you have more styles to specify, use `static_style_seq!` instead.
#[macro_export]
macro_rules! static_styles {
  ($a:ident, $b:expr) => {
    $crate::static_style_seq!($a, &[$b]);
  };
  // to allow optional trailing comma
  ($a:ident, $b:expr,) => {
    $crate::static_style_seq!($a, &[$b]);
  };
  ($a:ident, $b:expr, $c:expr) => {
    $crate::static_style_seq!($a, &[$b, $c]);
  };
  ($a:ident, $b:expr, $c:expr,) => {
    $crate::static_style_seq!($a, &[$b, $c]);
  };
  ($a:ident, $b:expr, $c:expr, $d:expr) => {
    $crate::static_style_seq!($a, &[$b, $c, $d]);
  };
  ($a:ident, $b:expr, $c:expr, $d:expr,) => {
    $crate::static_style_seq!($a, &[$b, $c, $d]);
  };
  ($a:ident, $b:expr, $c:expr, $d:expr, $e:expr) => {
    $crate::static_style_seq!($a, &[$b, $c, $d, $e]);
  };
  ($a:ident, $b:expr, $c:expr, $d:expr, $e:expr,) => {
    $crate::static_style_seq!($a, &[$b, $c, $d, $e]);
  };
  ($a:ident, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
    $crate::static_style_seq!($a, &[$b, $c, $d, $e, $f]);
  };
}
pub use static_styles;
