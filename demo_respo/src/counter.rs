use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use respo::{button, div, span, ui::ui_button, util, CssColor, DispatchFn, RespoEvent, RespoNode, RespoStyle, StatesTree};

use super::store::ActionOp;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct MainState {
  counted: i32,
}

pub fn comp_counter(states: &StatesTree, _counted: i32) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();

  let state = states.data.cast_or_default::<MainState>()?;
  let state2 = state.clone();
  let counted = state.counted;
  let cursor2 = cursor.clone();

  let on_inc = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);
    if let RespoEvent::Click { original_event, .. } = e {
      original_event.prevent_default();
    }

    dispatch.run(ActionOp::Increment)?;
    dispatch.run_state(
      &cursor,
      MainState {
        counted: state.counted + 2,
      },
    )?;
    Ok(())
  };

  let on_dec = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);
    dispatch.run(ActionOp::Decrement)?;
    dispatch.run_state(
      &cursor2,
      MainState {
        counted: state2.counted - 1,
      },
    )?;
    Ok(())
  };

  Ok(
    div()
      .children([
        div()
          .children([
            button()
              .class(ui_button())
              .inner_text("demo inc")
              .style(RespoStyle::default().margin(4.).to_owned())
              .on_click(on_inc)
              .to_owned(),
            button()
              .class(ui_button())
              .inner_text("demo dec")
              .style(RespoStyle::default().margin(4.).to_owned())
              .on_click(on_dec)
              .to_owned(),
          ])
          .to_owned(),
        div()
          .children([span()
            .inner_text(format!("value is: {}", counted))
            .style(
              RespoStyle::default()
                .color(CssColor::Hsluv(270, 100, 40))
                .font_family("Menlo".to_owned())
                .font_size(10. + counted as f32)
                .to_owned(),
            )
            .to_owned()])
          .to_owned(),
        div()
          .children([span().inner_text(format!("local state: {}", counted)).to_owned()])
          .to_owned(),
      ])
      .to_owned(),
  )
}
