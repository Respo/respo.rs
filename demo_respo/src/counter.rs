use std::fmt::Debug;

use respo::{button, div, span, ui::ui_button, util, CssColor, DispatchFn, RespoEvent, RespoNode, RespoStyle, StatesTree};

use super::store::ActionOp;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct MainState {
  counted: i32,
}

pub fn comp_counter(states: &StatesTree, _counted: i32) -> Result<RespoNode<ActionOp>, String> {
  let cursor = states.path();

  let state = states.data.cast_or_default::<MainState>()?;
  let counted = state.counted;

  let on_inc = {
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
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
    }
  };

  let on_dec = {
    let cursor = cursor.to_owned();
    let state = state.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);
      dispatch.run(ActionOp::Decrement)?;
      dispatch.run_state(
        &cursor,
        MainState {
          counted: state.counted - 1,
        },
      )?;
      Ok(())
    }
  };

  Ok(
    div()
      .children([
        div()
          .children([
            button()
              .class(ui_button())
              .inner_text("demo inc")
              .style(RespoStyle::default().margin(4.).end())
              .on_click(on_inc)
              .end(),
            button()
              .class(ui_button())
              .inner_text("demo dec")
              .style(RespoStyle::default().margin(4.).end())
              .on_click(on_dec)
              .end(),
          ])
          .end(),
        div()
          .children([span()
            .inner_text(format!("value is: {}", counted))
            .style(
              RespoStyle::default()
                .color(CssColor::Hsluv(270, 100, 40))
                .font_family("Menlo".to_owned())
                .font_size(10. + counted as f32)
                .end(),
            )
            .end()])
          .end(),
        div().children([span().inner_text(format!("local state: {}", counted)).end()]).end(),
      ])
      .end(),
  )
}
