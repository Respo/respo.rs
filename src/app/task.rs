use crate::respo::{CssColor, CssRule, RespoStyle};

/// turns a class-name to string
fn style_panel() -> String {
  let name = "style-panel";

  let styles = vec![(
    name.to_owned(),
    RespoStyle::default().add(CssRule::BackgroundColor(CssColor::Red)).to_owned(),
  )];
  let raw = RespoStyle::render_rules(&styles);

  name.to_owned()
}
