use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};

use crate::ui::dialog::{css_backdrop, css_button, css_modal_card};
use crate::ui::dialog::{EffectFocus, EffectModalFade, BUTTON_NAME};
use crate::ui::{column, ui_button, ui_center, ui_fullscreen, ui_global, ui_row_parted};

use crate::node::css::{CssLineHeight, CssPosition, RespoStyle};
use crate::node::{DispatchFn, RespoAction, RespoEvent, RespoNode};
use crate::{button, div, space, span, RespoComponent};

use crate::states_tree::{RespoState, RespoStatesTree};

use super::comp_esc_listener;

/// The options for alert modal.
#[derive(Debug, Clone, Default)]
pub struct AlertOptions {
  /// inline style for backdrop
  pub backdrop_style: RespoStyle,
  /// inline style for card
  pub card_style: RespoStyle,
  /// message of the alert modal, defaults to `Alert!`
  pub text: Option<String>,
  /// text on button
  pub button_text: Option<String>,
}

fn comp_alert_modal<T, U, V>(options: AlertOptions, show: bool, on_read: U, on_close: V) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let read = Rc::new(on_read);
  let close = Rc::new(on_close);

  Ok(
    RespoComponent::named(
      "alert-modal",
      div()
        .style(RespoStyle::default().position(CssPosition::Absolute))
        .elements([if show {
          div()
            .class_list(&[ui_fullscreen(), ui_center(), css_backdrop()])
            .style(options.backdrop_style)
            .on_click({
              let close = close.to_owned();
              move |e, dispatch| -> Result<(), String> {
                if let RespoEvent::Click { original_event, .. } = e {
                  // stop propagation to prevent closing the modal
                  original_event.stop_propagation();
                }
                close(dispatch)?;
                Ok(())
              }
            })
            .children([
              div()
                .class_list(&[column(), ui_global(), css_modal_card()])
                .style(RespoStyle::default().line_height(CssLineHeight::Px(32.0)))
                .style(options.card_style)
                .on_click(move |e, _dispatch| -> Result<(), String> {
                  // nothing to do
                  if let RespoEvent::Click { original_event, .. } = e {
                    // stop propagation to prevent closing the modal
                    original_event.stop_propagation();
                  }
                  Ok(())
                })
                .elements([div().elements([
                  span().inner_text(options.text.unwrap_or_else(|| "Alert!".to_owned())),
                  space(None, Some(8)),
                  div().class(ui_row_parted()).elements([
                    span(),
                    button()
                      .class_list(&[ui_button(), css_button(), BUTTON_NAME.to_owned()])
                      .inner_text(options.button_text.unwrap_or_else(|| "Read".to_owned()))
                      .on_click({
                        let close = close.to_owned();
                        move |_e, dispatch| -> Result<(), String> {
                          read(dispatch.to_owned())?;
                          close(dispatch)?;
                          Ok(())
                        }
                      }),
                  ]),
                ])])
                .to_node(),
              comp_esc_listener(show, close)?,
            ])
        } else {
          span().attribute("data-name", "placeholder")
        }]),
    )
    .effect(EffectFocus { show })
    .effect(EffectModalFade { show })
    .to_node()
    .rc(),
  )
}

/// provides the interfaces to component of alert dialog
pub trait AlertPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String>,
{
  /// renders virtual dom for alert modal
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show alert, second parameter is a message that could overwrite the default message
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String>;
  /// to close alert
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  /// show alert with options, `on_read` is the callback function when read button is clicked
  fn new(states: RespoStatesTree, options: AlertOptions, on_read: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;

  /// return referencial counted alert plugin
  fn share_with_ref(&self) -> Rc<Self>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize, RespoState)]
struct AlertPluginState {
  show: bool,
  text: Option<String>,
}

/// abstraction for Alert modal, new with `AlertOption`,
/// just displaying a message, you read it, you close it
#[derive(Debug, Clone)]
pub struct AlertPlugin<T, U>
where
  T: Clone + Debug,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
{
  state: Rc<AlertPluginState>,
  options: AlertOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<Rc<str>>,
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
    let cursor = &self.cursor;
    let state = &self.state;

    let mut options = self.options.to_owned();
    options.text = {
      let state = state.to_owned();
      state.text.as_deref().or(options.text.as_deref()).map(ToOwned::to_owned)
    };

    comp_alert_modal(
      options,
      self.state.show,
      {
        let cursor = cursor.to_owned();
        let state = state.to_owned();
        move |dispatch| {
          on_read(dispatch.to_owned())?;
          let s = AlertPluginState {
            show: false,
            text: state.text.to_owned(),
          };
          dispatch.run_state(&cursor, s)?;
          Ok(())
        }
      },
      {
        let cursor = cursor.to_owned();
        let state = state.to_owned();
        move |dispatch| {
          let s = AlertPluginState {
            show: false,
            text: state.text.to_owned(),
          };
          dispatch.run_state(&cursor, s)?;
          Ok(())
        }
      },
    )
  }
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String> {
    let s = AlertPluginState { show: true, text };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = AlertPluginState {
      show: false,
      text: self.text.to_owned(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: RespoStatesTree, options: AlertOptions, on_read: U) -> Result<Self, String> {
    let cursor = states.path();
    let state = states.cast_branch::<AlertPluginState>();

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

  fn share_with_ref(&self) -> Rc<Self> {
    Rc::new(self.to_owned())
  }
}
