use crate::respo::{
  css::*, declare_static_style, CssBoxSizing, CssColor, CssDisplay, CssLineHeight, CssSize, CssTextAlign, CssVerticalAlign, RespoStyle,
};

const DEFAULT_FONTS: &str = "Hind,Verdana,'Hiragino Sans GB','WenQuanYi Micro Hei','Microsoft Yahei',sans-serif";
const CODE_FONTS: &str = "Source Code Pro, Menlo, Ubuntu Mono, Consolas, monospace";
const FANCY_FONTS: &str = "Josefin Sans, Helvetica neue, Arial, sans-serif";
const NORMAL_FONTS: &str = "Hind, Helvatica, Arial, sans-serif";

pub fn ui_global() -> String {
  declare_static_style(
    "ui-global",
    &[
      (
        "$0",
        RespoStyle::default()
          .font_family(DEFAULT_FONTS.to_owned())
          .line_height(CssLineHeight::Em(2.))
          .font_size(14.)
          .color(CssColor::Hsla(0., 0., 20., 1.)),
      ),
      ("$0 *", RespoStyle::default().box_sizing(CssBoxSizing::BorderBox)),
    ],
  )
}

pub fn ui_input() -> String {
  declare_static_style(
    "ui-input",
    &[
      (
        "$0",
        RespoStyle::default()
          .border(Some((1., CssBorderStyle::Solid, CssColor::Hsla(0.0, 0.0, 80.0, 1.0))))
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
          .border(Some((1.0, CssBorderStyle::Solid, CssColor::Hsla(200.0, 50.0, 75.0, 1.0))))
          .box_shadow(0.0, 0.0, 4.0, 0.0, CssColor::Hsla(200.0, 70.0, 50.0, 0.2)),
      ),
    ],
  )
}

pub fn ui_button() -> String {
  declare_static_style(
    "ui-button",
    &[
      (
        "$0",
        RespoStyle::default()
          .outline(None)
          .background_color(CssColor::White)
          .border(Some((1., CssBorderStyle::Solid, CssColor::Hsla(200., 100., 76., 1.))))
          .min_width(CssSize::Px(80.))
          .line_height(CssLineHeight::Px(24.))
          .border_radius(4.)
          .font_size(14.)
          .cursor("pointer".to_owned())
          .transition_duration(200.)
          .text_align(CssTextAlign::Center),
      ),
      (
        "$0:hover",
        RespoStyle::default().background_color(CssColor::Hsla(0., 0.0, 98.0, 1.0)),
      ),
      (
        "$0:active",
        RespoStyle::default()
          .transform(CssTransform::Scale(1.04, 1.04))
          .transition_duration(0.0),
      ),
    ],
  )
}

pub fn ui_center() -> String {
  declare_static_style(
    "ui-center",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Column)
        .justify_content(CssFlexJustifyContent::Center)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_column() -> String {
  declare_static_style(
    "ui-column",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Column)
        .align_items(CssFlexAlignItems::Stretch),
    )],
  )
}

pub fn ui_column_dispersive() -> String {
  declare_static_style(
    "ui-column-dispersive",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Column)
        .justify_content(CssFlexJustifyContent::SpaceAround)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_column_evenly() -> String {
  declare_static_style(
    "ui-column-evenly",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Column)
        .justify_content(CssFlexJustifyContent::SpaceEvenly)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_column_parted() -> String {
  declare_static_style(
    "ui-column-parted",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Column)
        .justify_content(CssFlexJustifyContent::SpaceBetween)
        .align_items(CssFlexAlignItems::Stretch),
    )],
  )
}

pub fn ui_expand() -> String {
  declare_static_style("ui-expand", &[("$0", RespoStyle::default().insert("flex", "1".to_owned()))])
}

pub fn ui_fullscreen() -> String {
  declare_static_style(
    "ui-fullscreen",
    &[(
      "$0",
      RespoStyle::default()
        .position(CssPosition::Absolute)
        .left(CssSize::Px(0.))
        .top(CssSize::Px(0.))
        .width(CssSize::Percent(100.))
        .height(CssSize::Percent(100.))
        .overflow(CssOverflow::Auto),
    )],
  )
}

pub fn ui_font_code() -> String {
  declare_static_style("ui-font-code", &[("$0", RespoStyle::default().font_family(CODE_FONTS.to_owned()))])
}

pub fn ui_font_fancy() -> String {
  declare_static_style(
    "ui-font-fancy",
    &[("$0", RespoStyle::default().font_family(FANCY_FONTS.to_owned()))],
  )
}

pub fn ui_font_normal() -> String {
  declare_static_style(
    "ui-font-normal",
    &[("$0", RespoStyle::default().font_family(NORMAL_FONTS.to_owned()))],
  )
}

pub fn ui_textarea() -> String {
  declare_static_style(
    "ui-textarea",
    &[(
      "$0",
      RespoStyle::default()
        .outline(None)
        .font_size(14.)
        .font_family(DEFAULT_FONTS.to_owned())
        .border(Some((1., CssBorderStyle::Solid, CssColor::Hsla(0., 0., 20., 1.))))
        .border_radius(4.)
        .padding(8.)
        .min_width(CssSize::Px(240.))
        .vertical_align(CssVerticalAlign::Top),
    )],
  )
}

pub fn ui_link() -> String {
  declare_static_style(
    "ui-link",
    &[(
      "$0",
      RespoStyle::default()
        .text_decoration(CssTextDecoration::Underline)
        .insert("user-select", "no-select".to_owned())
        .height(CssSize::Px(24.))
        .line_height(CssLineHeight::Px(24.))
        .margin(4.)
        .display(CssDisplay::InlineBlock)
        .color(CssColor::Hsla(200., 100., 76., 1.))
        .cursor("pointer".to_owned()),
    )],
  )
}

pub fn ui_row() -> String {
  declare_static_style(
    "ui-row",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .align_items(CssFlexAlignItems::Stretch),
    )],
  )
}

pub fn ui_row_center() -> String {
  declare_static_style(
    "ui-row-center",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .justify_content(CssFlexJustifyContent::Center)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_row_dispersive() -> String {
  declare_static_style(
    "ui-row-dispersive",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .justify_content(CssFlexJustifyContent::SpaceAround)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_row_evenly() -> String {
  declare_static_style(
    "ui-row-evenly",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .justify_content(CssFlexJustifyContent::SpaceEvenly)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_row_middle() -> String {
  declare_static_style(
    "ui-row-middle",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .justify_content(CssFlexJustifyContent::FlexStart)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}

pub fn ui_row_parted() -> String {
  declare_static_style(
    "ui-row-parted",
    &[(
      "$0",
      RespoStyle::default()
        .display(CssDisplay::Flex)
        .flex_direction(CssFlexDirection::Row)
        .justify_content(CssFlexJustifyContent::SpaceBetween)
        .align_items(CssFlexAlignItems::Center),
    )],
  )
}
