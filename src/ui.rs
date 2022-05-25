use crate::respo::{CssBoxSizing, CssColor, CssDisplay, CssLineHeight, CssSize, CssTextAlign, CssVerticalAlign, RespoStyle, *};

const DEFAULT_FONTS: &str = "Hind,Verdana,'Hiragino Sans GB','WenQuanYi Micro Hei','Microsoft Yahei',sans-serif";
const CODE_FONTS: &str = "Source Code Pro, Menlo, Ubuntu Mono, Consolas, monospace";
const FANCY_FONTS: &str = "Josefin Sans, Helvetica neue, Arial, sans-serif";
const NORMAL_FONTS: &str = "Hind, Helvatica, Arial, sans-serif";

static_styles!(
  ui_global,
  (
    "$0",
    RespoStyle::default()
      .font_family(DEFAULT_FONTS.to_owned())
      .line_height(CssLineHeight::Em(2.))
      .font_size(14.)
      .color(CssColor::Hsl(0, 0, 20)),
  ),
  ("$0 *", RespoStyle::default().box_sizing(CssBoxSizing::BorderBox))
);

static_styles!(
  ui_input,
  (
    "$0",
    RespoStyle::default()
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(0, 0, 80))))
      .border_radius(4.)
      .padding4(4., 8., 4., 8.)
      .min_width(CssSize::Px(120.))
      .line_height(CssLineHeight::Em(1.5))
      .font_family(DEFAULT_FONTS.to_owned())
      .vertical_align(CssVerticalAlign::Middle),
  ),
  (
    "$0:focus",
    RespoStyle::default()
      .border(Some((1.0, CssBorderStyle::Solid, CssColor::Hsl(200, 50, 75))))
      .box_shadow(0.0, 0.0, 4.0, 0.0, CssColor::Hsl(200, 70, 50)),
  )
);

static_styles!(
  ui_button,
  (
    "$0",
    RespoStyle::default()
      .outline(None)
      .background_color(CssColor::White)
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(200, 100, 76))))
      .min_width(CssSize::Px(80.))
      .line_height(CssLineHeight::Px(24.))
      .border_radius(4.)
      .font_size(14.)
      .cursor("pointer".to_owned())
      .transition_duration(200.)
      .text_align(CssTextAlign::Center),
  ),
  ("$0:hover", RespoStyle::default().background_color(CssColor::Hsl(0, 0, 98))),
  (
    "$0:active",
    RespoStyle::default()
      .transform(CssTransform::Scale(1.04, 1.04))
      .transition_duration(0.0),
  ),
);

static_styles!(
  ui_center,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::Center)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_column,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(
  ui_column_dispersive,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceAround)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_column_evenly,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceEvenly)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_column_parted,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceBetween)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(ui_expand, ("$0", RespoStyle::default().insert("flex", "1".to_owned())));

static_styles!(
  ui_fullscreen,
  (
    "$0",
    RespoStyle::default()
      .position(CssPosition::Absolute)
      .left(CssSize::Px(0.))
      .top(CssSize::Px(0.))
      .width(CssSize::Percent(100.))
      .height(CssSize::Percent(100.))
      .overflow(CssOverflow::Auto),
  )
);

static_styles!(ui_font_code, ("$0", RespoStyle::default().font_family(CODE_FONTS.to_owned())));

static_styles!(ui_font_fancy, ("$0", RespoStyle::default().font_family(FANCY_FONTS.to_owned())));

static_styles!(ui_font_normal, ("$0", RespoStyle::default().font_family(NORMAL_FONTS.to_owned())));

static_styles!(
  ui_textarea,
  (
    "$0",
    RespoStyle::default()
      .outline(None)
      .font_size(14.)
      .font_family(DEFAULT_FONTS.to_owned())
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(0, 0, 20))))
      .border_radius(4.)
      .padding(8.)
      .min_width(CssSize::Px(240.))
      .vertical_align(CssVerticalAlign::Top),
  )
);

static_styles!(
  ui_link,
  (
    "$0",
    RespoStyle::default()
      .text_decoration(CssTextDecoration::Underline)
      .insert("user-select", "no-select".to_owned())
      .height(CssSize::Px(24.))
      .line_height(CssLineHeight::Px(24.))
      .margin(4.)
      .display(CssDisplay::InlineBlock)
      .color(CssColor::Hsl(200, 100, 76))
      .cursor("pointer".to_owned()),
  )
);

static_styles!(
  ui_row,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(
  ui_row_center,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::Center)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_dispersive,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceAround)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_evenly,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceEvenly)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_middle,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::FlexStart)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_parted,
  (
    "$0",
    RespoStyle::default()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceBetween)
      .align_items(CssFlexAlignItems::Center),
  )
);
