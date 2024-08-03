//! Some pre-defined styles for layout fonts, and form elements.
//! Highly coupled with styles from <http://ui.respo-mvc.org/> .
//!
//! - Layouts: flexbox rows and columns, also with centering
//! - Elements: button, input, textarea, and link from Respo UI.
//! - Fonts: fancy(Josefin Sans), normal(Hind), code(monospace from system).
//!
//! Since this is CSS rules, you can combine styles with:
//!
//! ```ignore
//! format!("{} {} {}", ui_input(), your_shared(), your_own())
//! ```

pub mod dialog;

use crate::{
  node::css::{CssBoxSizing, CssColor, CssDisplay, CssLineHeight, CssTextAlign, CssVerticalAlign, *},
  static_styles,
};

const DEFAULT_FONTS: &str = "Hind,Verdana,'Hiragino Sans GB','WenQuanYi Micro Hei','Microsoft Yahei',sans-serif";
const CODE_FONTS: &str = "Source Code Pro, Menlo, Ubuntu Mono, Consolas, monospace";
const FANCY_FONTS: &str = "Josefin Sans, Helvetica neue, Arial, sans-serif";
const NORMAL_FONTS: &str = "Hind, Helvatica, Arial, sans-serif";

static_styles!(
  ui_global,
  (
    "&",
    respo_style()
      .font_family(DEFAULT_FONTS.to_owned())
      .line_height(CssLineHeight::Em(2.))
      .font_size(14.)
      .color(CssColor::Hsl(0, 0, 20)),
  ),
  ("& *", respo_style().box_sizing(CssBoxSizing::BorderBox))
);

static_styles!(
  ui_input,
  (
    "&",
    respo_style()
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(0, 0, 80))))
      .border_radius(4.)
      .padding4(4, 8, 4, 8)
      .min_width(120.px())
      .line_height(CssLineHeight::Em(1.5))
      .font_family(DEFAULT_FONTS.to_owned())
      .vertical_align(CssVerticalAlign::Middle),
  ),
  (
    "&:focus",
    respo_style()
      .border(Some((1.0, CssBorderStyle::Solid, CssColor::Hsl(200, 50, 75))))
      .box_shadow(0.0, 0.0, 4.0, 0.0, CssColor::Hsl(200, 70, 50)),
  )
);

static_styles!(
  ui_button,
  (
    "&",
    respo_style()
      .outline(None)
      .background_color(CssColor::White)
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(200, 100, 76))))
      .min_width(80.px())
      .line_height(CssLineHeight::Px(24.))
      .border_radius(4.)
      .font_size(14.)
      .cursor("pointer")
      .transition_duration(200.)
      .text_align(CssTextAlign::Center),
  ),
  ("&:hover", respo_style().background_color(CssColor::Hsl(0, 0, 98))),
  (
    "&:active",
    respo_style().transform(CssTransform::Scale(1.02, 1.02)).transition_duration(0.0),
  ),
);

static_styles!(
  ui_button_primary,
  (
    "&",
    respo_style()
      .outline(None)
      .color(CssColor::White)
      .background_color(CssColor::Hsl(220, 80, 60))
      .border(Some((0., CssBorderStyle::Solid, CssColor::Hsl(220, 80, 60))))
      .min_width(80.px())
      .line_height(CssLineHeight::Px(24.))
      .border_radius(4.)
      .font_size(14.)
      .cursor("pointer")
      .transition_duration(200.)
      .text_align(CssTextAlign::Center),
  ),
  ("&:hover", respo_style().background_color(CssColor::Hsl(220, 80, 64))),
  (
    "&:active",
    respo_style()
      .transform(CssTransform::Scale(1.02, 1.02))
      .background_color(CssColor::Hsl(220, 80, 68))
      .transition_duration(0.0),
  ),
);

static_styles!(
  ui_button_danger,
  (
    "&",
    respo_style()
      .outline(None)
      .color(CssColor::White)
      .background_color(CssColor::Hsl(6, 100, 60))
      .border(Some((0., CssBorderStyle::Solid, CssColor::Hsl(6, 100, 60))))
      .min_width(80.px())
      .line_height(CssLineHeight::Px(24.))
      .border_radius(4.)
      .font_size(14.)
      .cursor("pointer")
      .transition_duration(200.)
      .text_align(CssTextAlign::Center),
  ),
  ("&:hover", respo_style().background_color(CssColor::Hsl(6, 100, 64))),
  (
    "&:active",
    respo_style()
      .transform(CssTransform::Scale(1.02, 1.02))
      .background_color(CssColor::Hsl(6, 100, 68))
      .transition_duration(0.0),
  ),
);

static_styles!(
  ui_center,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::Center)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  column,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(
  ui_column_dispersive,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceAround)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_column_evenly,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceEvenly)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_column_parted,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Column)
      .justify_content(CssFlexJustifyContent::SpaceBetween)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(ui_expand, ("&", respo_style().insert("flex", "1".to_owned())));

static_styles!(
  ui_fullscreen,
  (
    "&",
    respo_style()
      .position(CssPosition::Absolute)
      .left(0.px())
      .top(0.px())
      .width(100.percent())
      .height(100.percent())
      .overflow(CssOverflow::Auto),
  )
);

static_styles!(ui_font_code, ("&", respo_style().font_family(CODE_FONTS.to_owned())));

static_styles!(ui_font_fancy, ("&", respo_style().font_family(FANCY_FONTS.to_owned())));

static_styles!(ui_font_normal, ("&", respo_style().font_family(NORMAL_FONTS.to_owned())));

static_styles!(
  ui_textarea,
  (
    "&",
    respo_style()
      .outline(None)
      .font_size(14.)
      .font_family(DEFAULT_FONTS.to_owned())
      .border(Some((1., CssBorderStyle::Solid, CssColor::Hsl(0, 0, 20))))
      .border_radius(4.)
      .padding(8)
      .min_width(240.px())
      .vertical_align(CssVerticalAlign::Top),
  )
);

static_styles!(
  ui_link,
  (
    "&",
    respo_style()
      .text_decoration(CssTextDecoration::Underline)
      .insert("user-select", "no-select".to_owned())
      .height(24.px())
      .line_height(CssLineHeight::Px(24.))
      .margin(4)
      .display(CssDisplay::InlineBlock)
      .color(CssColor::Hsl(200, 100, 76))
      .cursor("pointer"),
  )
);

static_styles!(
  ui_row,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .align_items(CssFlexAlignItems::Stretch),
  )
);

static_styles!(
  ui_row_center,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::Center)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_dispersive,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceAround)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_evenly,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceEvenly)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_middle,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::FlexStart)
      .align_items(CssFlexAlignItems::Center),
  )
);

static_styles!(
  ui_row_parted,
  (
    "&",
    respo_style()
      .display(CssDisplay::Flex)
      .flex_direction(CssFlexDirection::Row)
      .justify_content(CssFlexJustifyContent::SpaceBetween)
      .align_items(CssFlexAlignItems::Center),
  )
);
