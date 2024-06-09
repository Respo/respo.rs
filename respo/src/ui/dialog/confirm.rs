use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use js_sys::{Array, Function, Reflect};
use respo_state_derive::RespoState;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::ui::dialog::{css_backdrop, css_button, css_modal_card};
use crate::ui::{column, ui_button, ui_center, ui_fullscreen, ui_global, ui_row_parted};

use crate::node::css::{CssLineHeight, CssPosition, RespoStyle};
use crate::node::{DispatchFn, RespoAction, RespoEvent, RespoNode};
use crate::{app, button, div, space, span, RespoComponent};

use crate::states_tree::{RespoState, StatesTree};

use crate::ui::dialog::{effect_focus, effect_modal_fade, BUTTON_NAME};

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

  Ok(
    RespoComponent::named(
      "confirm-modal",
      div()
        .style(RespoStyle::default().position(CssPosition::Absolute))
        .children([if show {
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
                .children([div()
                  .children([
                    span()
                      .inner_text(options.text.unwrap_or_else(|| "Need confirmation...".to_owned()))
                      .to_node(),
                    space(None, Some(8)).to_node(),
                    div()
                      .class(ui_row_parted())
                      .children([
                        span().to_node(),
                        button()
                          .class_list(&[ui_button(), css_button(), BUTTON_NAME.to_owned()])
                          .inner_text(options.button_text.unwrap_or_else(|| "Confirm".to_owned()))
                          .on_click({
                            let close = close.to_owned();
                            move |_e, dispatch| -> Result<(), String> {
                              confirm(dispatch.to_owned())?;
                              close(dispatch)?;
                              Ok(())
                            }
                          })
                          .to_node(),
                      ])
                      .to_node(),
                  ])
                  .to_node()])
                .to_node(),
              comp_esc_listener(show, close)?,
            ])
            .to_node()
        } else {
          span().attribute("data-name", "placeholder").to_node()
        }])
        .to_node(),
    )
    .effect(&[show], effect_focus)
    .effect(&[show], effect_modal_fade)
    .to_node()
    .rc(),
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

  fn rc(&self) -> Rc<Self>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, RespoState)]
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
  state: Rc<ConfirmPluginState>,
  options: ConfirmOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<Rc<str>>,
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
    let cursor = self.cursor.to_owned();
    let state = self.state.to_owned();

    comp_confirm_modal(
      self.options.to_owned(),
      state.show.to_owned(),
      {
        let c = cursor.to_owned();
        let st = state.to_owned();
        move |dispatch| {
          on_confirm(dispatch.to_owned())?;
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
            app::util::log!("next task is None");
          };
          let s = ConfirmPluginState {
            show: false,
            text: st.text.to_owned(),
          };
          dispatch.run_state(&c, s)?;
          // clean up leaked closure
          let window = web_sys::window().expect("window");
          let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
          Ok(())
        }
      },
      {
        let st = state.to_owned();
        let c = cursor.to_owned();
        move |dispatch| {
          let s = ConfirmPluginState {
            show: false,
            text: st.text.to_owned(),
          };
          dispatch.run_state(&c, s)?;
          // clean up leaked closure
          let window = web_sys::window().expect("window");
          let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
          Ok(())
        }
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
      app::util::log!("failed to store next task {:?}", e);
    }
    task.forget();
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = ConfirmPluginState {
      show: false,
      text: self.text.to_owned(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: ConfirmOptions, on_confirm: U) -> Result<Self, String> {
    let cursor = states.path();
    let state = states.cast_branch::<ConfirmPluginState>()?;

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

  // return a reference counted instance
  fn rc(&self) -> Rc<Self> {
    Rc::new(self.to_owned())
  }
}
