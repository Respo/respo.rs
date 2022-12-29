use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use js_sys::{Array, Function, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::dialog::{css_backdrop, css_button, css_modal_card};
use crate::ui::{ui_button, ui_center, ui_column, ui_fullscreen, ui_global, ui_row_parted};

use crate::{
  button, div, respo, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree,
};

use crate::dialog::{effect_focus, effect_modal_fade, BUTTON_NAME};

use super::comp_esc_listener;

const NEXT_TASK_NAME: &str = "_RESPO_CONFIRM_NEXT_TASK";

/// options for confirm dialog
#[derive(Debug, Clone, Default)]
pub struct ConfirmOptions {
  /// inline style for backdrop
  pub backdrop_style: RespoStyle,
  /// inline style for card
  card_style: RespoStyle,
  /// message to display
  text: Option<String>,
  /// text on button
  button_text: Option<String>,
}

fn comp_confirm_modal<T, U, V>(options: ConfirmOptions, show: bool, on_confirm: U, on_close: V) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let confirm = Rc::new(on_confirm);
  let close = Rc::new(on_close);
  let close2 = close.clone();
  let close3 = close.clone();

  Ok(
    RespoNode::new_component(
      "confirm-modal",
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
            .children([
              div()
                .class_list(&[ui_column(), ui_global(), css_modal_card()])
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
                    span()
                      .inner_text(options.text.unwrap_or_else(|| "Need confirmation...".to_owned()))
                      .to_owned(),
                    space(None, Some(8)),
                    div()
                      .class(ui_row_parted())
                      .children([
                        span(),
                        button()
                          .class_list(&[ui_button(), css_button(), BUTTON_NAME.to_owned()])
                          .inner_text(options.button_text.unwrap_or_else(|| "Confirm".to_owned()))
                          .on_click(move |_e, dispatch| -> Result<(), String> {
                            let d2 = dispatch.clone();
                            confirm(dispatch)?;
                            close2(d2)?;
                            Ok(())
                          })
                          .to_owned(),
                      ])
                      .to_owned(),
                  ])
                  .to_owned()])
                .to_owned(),
              comp_esc_listener(show, close3)?,
            ])
            .to_owned()
        } else {
          span().attribute("data-name", "placeholder").to_owned()
        }])
        .to_owned(),
    )
    .effect(&[show], effect_focus)
    .effect(&[show], effect_modal_fade)
    .share_with_ref(),
  )
}

/// provides the interfaces to component of confirm dialog
pub trait ConfirmPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String>,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show dialog, second parameter is a callback when confirmed,
  /// the callback is implemented dirty, it perform directly after confirmed
  fn show<V>(&self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn() -> Result<(), String> + 'static;
  /// to close dialog
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  /// creates a new instance of confirm plugin, second parameter is a callback when confirmed
  fn new(states: StatesTree, options: ConfirmOptions, on_confirm: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;

  fn share_with_ref(&self) -> Rc<Self>;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ConfirmPluginState {
  show: bool,
  text: Option<String>,
}

/// Popup a confirmation dialog, confirm to process next task
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
  phantom: PhantomData<T>,
}

impl<T, U> ConfirmPluginInterface<T, U> for ConfirmPlugin<T, U>
where
  T: Clone + Debug + RespoAction,
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static + Copy,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let on_confirm = self.on_confirm;
    let cursor = self.cursor.clone();
    let cursor2 = self.cursor.clone();
    let state = self.state.to_owned();
    let state2 = self.state.to_owned();

    comp_confirm_modal(
      self.options.to_owned(),
      self.state.show,
      move |dispatch| {
        let d2 = dispatch.clone();
        on_confirm(dispatch)?;
        let window = web_sys::window().expect("window");
        // TODO dirty global variable
        let task = Reflect::get(&window, &JsValue::from_str(NEXT_TASK_NAME));
        if let Ok(f) = task {
          if f.is_function() {
            let f = f.dyn_into::<Function>().unwrap();
            let _ = f.apply(&JsValue::NULL, &Array::new());
          } else {
            return Err("_NEXT_TASK is not a function".to_owned());
          }
        } else {
          respo::util::log!("next task is None");
        };
        let s = ConfirmPluginState {
          show: false,
          text: state.text.to_owned(),
        };
        d2.run_state(&cursor, s)?;
        // clean up leaked closure
        let window = web_sys::window().expect("window");
        let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
        Ok(())
      },
      move |dispatch| {
        let s = ConfirmPluginState {
          show: false,
          text: state2.text.to_owned(),
        };
        dispatch.run_state(&cursor2, s)?;
        // clean up leaked closure
        let window = web_sys::window().expect("window");
        let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
        Ok(())
      },
    )
  }
  fn show<V>(&self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn() -> Result<(), String> + 'static,
  {
    let s = ConfirmPluginState {
      show: true,
      text: self.state.text.to_owned(),
    };
    let task = Closure::once(next_task);
    let window = web_sys::window().unwrap();
    // dirty global variable to store a shared callback
    if let Err(e) = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), task.as_ref()) {
      respo::util::log!("failed to store next task {:?}", e);
    }
    task.forget();
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

  fn new(states: StatesTree, options: ConfirmOptions, on_confirm: U) -> Result<Self, String> {
    let cursor = states.path();
    let state: ConfirmPluginState = states.data.cast_or_default()?;

    let instance = Self {
      state,
      options,
      text: None,
      cursor,
      on_confirm,
      phantom: PhantomData,
    };

    Ok(instance)
  }

  fn share_with_ref(&self) -> Rc<Self> {
    Rc::new(self.clone())
  }
}
