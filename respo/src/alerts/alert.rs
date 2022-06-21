use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::alerts::{css_backdrop, css_button, css_card};
use crate::ui::{ui_button, ui_center, ui_column, ui_fullscreen, ui_global, ui_row_parted};

use crate::{
  button, div, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree,
};

use crate::alerts::{effect_fade, effect_focus, BUTTON_NAME};

#[derive(Debug, Clone, Default)]
pub struct AlertOptions {
  backdrop_style: RespoStyle,
  card_style: RespoStyle,
  text: Option<String>,
  button_text: Option<String>,
}

pub fn comp_alert_modal<T, U, V>(options: AlertOptions, show: bool, on_read: U, on_close: V) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let read = Rc::new(on_read);
  let close = Rc::new(on_close);
  let close2 = close.clone();

  Ok(
    RespoNode::new_component(
      "alert-model",
      div()
        .style(RespoStyle::default().position(CssPosition::Absolute).to_owned())
        .children([if show {
          div()
            .class_list(&[ui_fullscreen(), ui_center(), css_backdrop()])
            .style(options.backdrop_style)
            .on_click(move |e, dispatch| -> Result<(), String> {
              if let RespoEvent::Click { original_event, .. } = e {
                // stop propagation to prevent closing the modal
                original_event.stop_propagation();
              }
              close(dispatch)?;
              Ok(())
            })
            .children([div()
              .class_list(&[ui_column(), ui_global(), css_card()])
              .style(RespoStyle::default().line_height(CssLineHeight::Px(32.0)).to_owned())
              .style(options.card_style)
              .on_click(move |e, _dispatch| -> Result<(), String> {
                // nothing to do
                if let RespoEvent::Click { original_event, .. } = e {
                  // stop propagation to prevent closing the modal
                  original_event.stop_propagation();
                }
                Ok(())
              })
              .children([div()
                .children([
                  span().inner_text(options.text.unwrap_or_else(|| "Alert!".to_owned())).to_owned(),
                  space(None, Some(8)),
                  div()
                    .class(ui_row_parted())
                    .children([
                      span(),
                      button()
                        .class_list(&[ui_button(), css_button(), BUTTON_NAME.to_owned()])
                        .inner_text(options.button_text.unwrap_or_else(|| "Read".to_owned()))
                        .on_click(move |_e, dispatch| -> Result<(), String> {
                          let d2 = dispatch.clone();
                          read(dispatch)?;
                          close2(d2)?;
                          Ok(())
                        })
                        .to_owned(),
                    ])
                    .to_owned(),
                ])
                .to_owned()])
              .to_owned()])
            .to_owned()
        } else {
          span().attribute("data-name", "placeholder").to_owned()
        }])
        .to_owned(),
    )
    .effect(&[show], effect_focus)
    .effect(&[show], effect_fade)
    .share_with_ref(),
  )
}

/// provides the interfaces to component of alert
pub trait AlertPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String>,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show alert
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String>;
  /// to close alert
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  fn new(states: StatesTree, options: AlertOptions, on_read: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AlertPluginState {
  show: bool,
  text: Option<String>,
}

/// struct for AlertPlugin
#[derive(Debug, Clone)]
pub struct AlertPlugin<T, U>
where
  T: Clone + Debug,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
{
  state: AlertPluginState,
  options: AlertOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<String>,
  on_read: U,
  phantom: PhantomData<T>,
}

impl<T, U> AlertPluginInterface<T, U> for AlertPlugin<T, U>
where
  T: Clone + Debug + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static + Copy,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let on_read = self.on_read;
    let cursor = self.cursor.clone();
    let cursor2 = self.cursor.clone();
    let state = self.state.to_owned();
    let state2 = self.state.to_owned();
    comp_alert_modal(
      self.options.to_owned(),
      self.state.show,
      move |dispatch| {
        let d2 = dispatch.clone();
        on_read(dispatch)?;
        let s = AlertPluginState {
          show: false,
          text: state.text.to_owned(),
        };
        d2.run_state(&cursor, s)?;
        Ok(())
      },
      move |dispatch| {
        let s = AlertPluginState {
          show: false,
          text: state2.text.to_owned(),
        };
        dispatch.run_state(&cursor2, s)?;
        Ok(())
      },
    )
  }
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String> {
    let s = AlertPluginState {
      show: true,
      text: text.or_else(|| self.state.text.to_owned()),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = AlertPluginState {
      show: false,
      text: self.text.clone(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: AlertOptions, on_read: U) -> Result<Self, String> {
    let cursor = states.path();
    let state: AlertPluginState = states.data.cast_or_default()?;

    let instance = Self {
      state,
      options,
      text: None,
      cursor,
      on_read,
      phantom: PhantomData,
    };

    Ok(instance)
  }
}
