use crate::respo::{declare_static_style, CssBoxSizing, CssColor, CssLineHeight, CssSize, CssTextAlign, CssVerticalAlign, RespoStyle};

const DEFAULT_FONTS: &str = "Hind,Verdana,'Hiragino Sans GB','WenQuanYi Micro Hei','Microsoft Yahei',sans-serif";

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
    &[(
      "$0",
      RespoStyle::default()
        .border(None)
        .border_radius(4.)
        .padding4(4., 8., 4., 8.)
        .min_width(CssSize::Px(120.))
        .line_height(CssLineHeight::Em(1.5))
        .font_family(DEFAULT_FONTS.to_owned())
        .vertical_align(CssVerticalAlign::Middle),
    )],
  )
}

pub fn ui_button() -> String {
  declare_static_style(
    "ui-button",
    &[(
      "$0",
      RespoStyle::default()
        .min_width(CssSize::Px(80.))
        .line_height(CssLineHeight::Px(24.))
        .border_radius(4.)
        .font_size(14.)
        .text_align(CssTextAlign::Center),
    )],
  )
}
