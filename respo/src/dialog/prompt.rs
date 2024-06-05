// use std::borrow::Borrow;
use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use js_sys::{Array, Function, Reflect};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::dialog::{css_backdrop, css_button, css_modal_card};
use crate::ui::{column, ui_button, ui_center, ui_fullscreen, ui_global, ui_input, ui_row_parted, ui_textarea};

use crate::{
  button, div, input, respo, space, span, static_styles, textarea, CssColor, CssLineHeight, CssPosition, CssSize, DispatchFn,
  RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree,
};

use crate::dialog::{effect_modal_fade, BUTTON_NAME};

use super::comp_esc_listener;

const NEXT_TASK_NAME: &str = "_RESPO_PROMPT_NEXT_TASK";

/// options for prompt dialog
#[derive(Debug, Clone, Default)]
pub struct PromptOptions {
  /// inline style for backdrop
  pub backdrop_style: RespoStyle,
  /// inline style for card
  pub card_style: RespoStyle,
  /// hint to display, defaults `input message`
  pub text: Option<String>,
  /// text on button
  pub button_text: Option<String>,
  /// initial value of input
  pub initial_value: Option<String>,
  /// textarea or input
  pub multilines: bool,
  /// inline style for input bix
  pub input_style: RespoStyle,
  /// a validation function to check input
  pub validator: Option<PromptValidator>,
}

/// wraps validator function
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct PromptValidator(Rc<dyn Fn(&str) -> Result<(), String>>);

impl Debug for PromptValidator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(&PromptValidator ..)")
  }
}

impl PromptValidator {
  pub fn new<F>(f: F) -> Self
  where
    F: Fn(&str) -> Result<(), String> + 'static,
  {
    PromptValidator(Rc::new(f))
  }
  fn run(&self, value: &str) -> Result<(), String> {
    self.0(value)
  }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct InputState {
  draft: String,
  error: Option<String>,
}

fn comp_prompt_modal<T, U, V>(
  states: StatesTree,
  options: PromptOptions,
  show: bool,
  on_submit: U,
  on_close: V,
) -> Result<RespoNode<T>, String>
where
  U: Fn(String, DispatchFn<T>) -> Result<(), String> + 'static,
  V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug + RespoAction,
{
  let cursor = states.path();
  let mut state = states.data.cast_or_default::<InputState>()?;
  if let Some(text) = &options.initial_value {
    state = Rc::new(InputState {
      draft: text.to_string(),
      error: None,
    });
  }

  // respo::util::log!("State: {:?}", state);

  let submit = Rc::new(on_submit);
  let close = Rc::new(on_close);

  let on_text_input = {
    let cursor = cursor.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      if let RespoEvent::Input { value, .. } = e {
        dispatch.run_state(&cursor, InputState { draft: value, error: None })?;
      }
      Ok(())
    }
  };

  let check_submit = {
    let close = close.to_owned();
    let cursor = cursor.to_owned();
    move |text: &str, dispatch: DispatchFn<_>| -> Result<(), String> {
      respo::util::log!("validator: {:?}", &options.validator);
      if let Some(validator) = &options.validator {
        // let validator = validator.borrow();
        let result = validator.run(text);
        match result {
          Ok(()) => {
            submit(text.to_owned(), dispatch.to_owned())?;
            close(dispatch.to_owned())?;
            dispatch.to_owned().run_empty_state(&cursor)?;
          }
          Err(message) => {
            // dispatch.run_state(&cursor, InputState { draft: text.to_owned() })?;
            dispatch.run_state(
              &cursor,
              InputState {
                draft: text.to_owned(),
                error: Some(message),
              },
            )?;
          }
        }
      } else {
        submit(text.to_owned(), dispatch.to_owned())?;
        close(dispatch.to_owned())?;
        dispatch.run_empty_state(&cursor)?;
      }
      Ok(())
    }
  };

  let mut input_el = if options.multilines {
    textarea().class(ui_textarea()).to_owned()
  } else {
    input().class(ui_input()).to_owned()
  };

  Ok(
    RespoNode::new_component(
      "prompt-modal",
      div()
        .style(RespoStyle::default().position(CssPosition::Absolute).to_owned())
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
                {
                  let dispatch = dispatch.to_owned();
                  close(dispatch)?;
                }
                dispatch.run_empty_state(&cursor)?;
                Ok(())
              }
            })
            .children([
              div()
                .class_list(&[column(), ui_global(), css_modal_card()])
                .style(RespoStyle::default().line_height(CssLineHeight::Px(32.0)).to_owned())
                .style(options.card_style)
                .style(options.input_style)
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
                      .inner_text(options.text.unwrap_or_else(|| "Input your text:".to_owned()))
                      .to_owned(),
                    space(None, Some(8)),
                    div()
                      .children([input_el
                        .class_list(&[ui_input()])
                        .style(RespoStyle::default().width(CssSize::Percent(100.0)).to_owned())
                        .attribute("placeholder", "Content...")
                        .attribute("autoFocus", "autofocus")
                        .value(state.draft.to_owned())
                        .on_input(on_text_input)
                        .to_owned()])
                      .to_owned(),
                    match &state.error {
                      Some(message) => div().class_list(&[css_error()]).inner_text(message).to_owned(),
                      None => span(),
                    },
                    space(None, Some(8)),
                    div()
                      .class(ui_row_parted())
                      .children([
                        span(),
                        button()
                          .class_list(&[ui_button(), css_button(), BUTTON_NAME.to_owned()])
                          .inner_text(options.button_text.unwrap_or_else(|| "Submit".to_owned()))
                          .on_click(move |_e, dispatch| -> Result<(), String> {
                            check_submit(&state.draft, dispatch)?;
                            Ok(())
                          })
                          .to_owned(),
                      ])
                      .to_owned(),
                  ])
                  .to_owned()])
                .to_owned(),
              comp_esc_listener(show, close)?,
            ])
            .to_owned()
        } else {
          span().attribute("data-name", "placeholder").to_owned()
        }])
        .to_owned(),
    )
    // .effect(&[show], effect_focus)
    .effect(&[show], effect_modal_fade)
    .share_with_ref(),
  )
}

/// provides the interfaces to component of prompt dialog
pub trait PromptPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(String, DispatchFn<T>) -> Result<(), String>,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show prompt dialog, second parameter is the callback task when the dialog is read,
  /// the callback is stored in a dirty to provide syntax sugar
  fn show<V>(&self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn(String) -> Result<(), String> + 'static;
  /// to close prompt dialog
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  /// initialize the plugin, second parameter is the callback task when submitted,
  fn new(states: StatesTree, options: PromptOptions, on_submit: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;

  /// shared it in `Rc`
  fn share_with_ref(&self) -> Rc<Self>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct PromptPluginState {
  show: bool,
  text: Option<String>,
}

/// a dialog for prompt, request for some input, and submit
#[derive(Debug, Clone)]
pub struct PromptPlugin<T, U>
where
  T: Clone + Debug,
  U: Fn(String, DispatchFn<T>) -> Result<(), String> + 'static,
{
  states: StatesTree,
  state: Rc<PromptPluginState>,
  options: PromptOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<Rc<str>>,
  on_submit: U,
  phantom: PhantomData<T>,
}

impl<T, U> PromptPluginInterface<T, U> for PromptPlugin<T, U>
where
  T: Clone + Debug + RespoAction,
  U: Fn(String, DispatchFn<T>) -> Result<(), String> + 'static + Copy,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let on_submit = self.on_submit;
    let cursor = self.cursor.to_owned();
    let state = self.state.to_owned();

    comp_prompt_modal(
      self.states.pick("plugin"),
      self.options.to_owned(),
      self.state.show,
      {
        let cursor = cursor.to_owned();
        let state = state.to_owned();
        move |content, dispatch| {
          on_submit(content.to_owned(), dispatch.to_owned())?;
          let window = web_sys::window().expect("window");
          // TODO dirty global variable
          let task = Reflect::get(&window, &JsValue::from_str(NEXT_TASK_NAME));
          if let Ok(f) = task {
            if f.is_function() {
              let f = f.dyn_into::<Function>().unwrap();
              let arr = Array::new();
              arr.push(&JsValue::from_str(&content));
              let _ = f.apply(&JsValue::NULL, &arr);
            } else {
              return Err("_NEXT_TASK is not a function".to_owned());
            }
          } else {
            respo::util::log!("next task is None");
          };
          let s = PromptPluginState {
            show: false,
            text: state.text.to_owned(),
          };
          dispatch.run_state(&cursor, s)?;
          // clean up leaked closure
          let window = web_sys::window().expect("window");
          let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
          Ok(())
        }
      },
      move |dispatch| {
        let s = PromptPluginState {
          show: false,
          text: state.text.to_owned(),
        };
        dispatch.run_state(&cursor, s)?;
        // clean up leaked closure
        let window = web_sys::window().expect("window");
        let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
        Ok(())
      },
    )
  }
  fn show<V>(&self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn(String) -> Result<(), String> + 'static,
  {
    let s = PromptPluginState {
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
    let s = PromptPluginState {
      show: false,
      text: self.text.to_owned(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: PromptOptions, on_submit: U) -> Result<Self, String> {
    let cursor = states.path();
    let state = states.data.cast_or_default::<PromptPluginState>()?;

    let instance = Self {
      states,
      state,
      options,
      text: None,
      cursor,
      on_submit,
      phantom: PhantomData,
    };

    Ok(instance)
  }

  fn share_with_ref(&self) -> Rc<Self> {
    Rc::new(self.to_owned())
  }
}

static_styles!(css_error, ("&", RespoStyle::default().color(CssColor::Red)));
