use std::borrow::Borrow;
use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::alerts::{css_backdrop, css_button, css_card};
use crate::ui::{ui_button, ui_center, ui_column, ui_fullscreen, ui_global, ui_row_parted};

use crate::{
  button, div, respo, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree,
};

use crate::alerts::{effect_fade, effect_focus, BUTTON_NAME};

#[derive(Debug, Clone, Default)]
pub struct ConfirmOptions {
  backdrop_style: RespoStyle,
  card_style: RespoStyle,
  text: Option<String>,
  button_text: Option<String>,
}

pub fn comp_confirm_modal<T, U, V>(options: ConfirmOptions, show: bool, on_read: U, on_close: V) -> Result<RespoNode<T>, String>
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
              .on_click(move |_e, _dispatch| -> Result<(), String> {
                // nothing to do
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
pub trait ConfirmPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String>,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show alert
  fn show<V>(&mut self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn() -> Result<(), String> + 'static;
  /// to close alert
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  fn new(states: StatesTree, options: ConfirmOptions, on_read: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ConfirmPluginState {
  show: bool,
  text: Option<String>,
}

/// struct for ConfirmPlugin
#[derive(Debug, Clone)]
pub struct ConfirmPlugin<T, U>
where
  T: Clone + Debug,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
{
  state: ConfirmPluginState,
  options: ConfirmOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<String>,
  on_confirm: U,
  next_task: NextTask,
  phantom: PhantomData<T>,
}

#[derive(Clone, Default)]
pub struct NextTask {
  task: Option<Rc<dyn Fn() -> Result<(), String> + 'static>>,
}

impl Debug for NextTask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(&NextTask ..)")
  }
}

impl NextTask {
  pub fn new(task: impl Fn() -> Result<(), String> + 'static) -> Self {
    NextTask { task: Some(Rc::new(task)) }
  }
  pub fn none() -> Self {
    NextTask { task: None }
  }
}

impl<T, U> ConfirmPluginInterface<T, U> for ConfirmPlugin<T, U>
where
  T: Clone + Debug + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static + Copy,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let on_read = self.on_confirm;
    let cursor = self.cursor.clone();
    let cursor2 = self.cursor.clone();
    let state = self.state.to_owned();
    let state2 = self.state.to_owned();
    let next_task = self.next_task.task.clone();
    comp_confirm_modal(
      self.options.to_owned(),
      self.state.show,
      move |dispatch| {
        let d2 = dispatch.clone();
        on_read(dispatch)?;
        if let Some(f) = next_task.borrow() {
          f()?;
        } else {
          respo::util::log!("next task is None");
        };
        let s = ConfirmPluginState {
          show: false,
          text: state.text.to_owned(),
        };
        d2.run_state(&cursor, s)?;
        Ok(())
      },
      move |dispatch| {
        let s = ConfirmPluginState {
          show: false,
          text: state2.text.to_owned(),
        };
        dispatch.run_state(&cursor2, s)?;
        Ok(())
      },
    )
  }
  fn show<V>(&mut self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn() -> Result<(), String> + 'static,
  {
    let s = ConfirmPluginState {
      show: true,
      text: self.state.text.to_owned(),
    };
    self.next_task = NextTask::new(next_task);
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = ConfirmPluginState {
      show: false,
      text: self.text.clone(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: ConfirmOptions, on_read: U) -> Result<Self, String> {
    let cursor = states.path();
    let state: ConfirmPluginState = states.data.cast_or_default()?;

    let instance = Self {
      state,
      options,
      text: None,
      cursor,
      on_confirm: on_read,
      phantom: PhantomData,
      next_task: NextTask::none(),
    };

    Ok(instance)
  }
}
