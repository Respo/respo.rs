use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::{
  respo::{button, div, span, util, CssColor, RespoEvent, RespoNode, RespoStyle, StatesTree},
  ui::ui_button,
};

use super::data_types::ActionOp;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MainState {
  counted: i32,
}

pub fn comp_counter(states: &StatesTree, counted: i32) -> RespoNode<ActionOp> {
  let cursor = states.path();

  let state: MainState = match &states.data {
    Some(v) => serde_json::from_value(v.to_owned()).expect("to main state"),
    None => MainState::default(),
  };

  div()
    .add_children([
      div()
        .add_children([
          button()
            .class(ui_button())
            .inner_text("demo inc")
            .add_style(RespoStyle::default().margin(4.).to_owned())
            .on_click(move |e, dispatch| -> Result<(), String> {
              util::log!("click {:?}", e);
              if let RespoEvent::Click { original_event, .. } = e {
                original_event.prevent_default();
              }

              dispatch.run(ActionOp::Increment)?;
              dispatch.run(ActionOp::StatesChange(
                cursor.to_owned(),
                Some(
                  serde_json::to_value(MainState {
                    counted: state.counted + 2,
                  })
                  .expect("to json"),
                ),
              ))?;
              Ok(())
            })
            .to_owned(),
          button()
            .class(ui_button())
            .inner_text("demo dec")
            .add_style(RespoStyle::default().margin(4.).to_owned())
            .on_click(move |e, dispatch| -> Result<(), String> {
              util::log!("click {:?}", e);
              dispatch.run(ActionOp::Decrement)?;
              Ok(())
            })
            .to_owned(),
        ])
        .to_owned(),
      div()
        .add_children([span()
          .inner_text(format!("value is: {}", counted))
          .add_style(
            RespoStyle::default()
              .color(CssColor::Hsluv(270, 100, 40))
              .font_family("Menlo".to_owned())
              .font_size(10. + counted as f32)
              .to_owned(),
          )
          .to_owned()])
        .to_owned(),
      div()
        .add_children([span().inner_text(format!("local state: {}", state.counted)).to_owned()])
        .to_owned(),
    ])
    .to_owned()
}
