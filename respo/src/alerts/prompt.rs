use std::fmt::Debug;

use std::marker::PhantomData;
use std::option;
use std::rc::Rc;

use js_sys::{Array, Function, Reflect};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};

use crate::alerts::{css_backdrop, css_button, css_card};
use crate::ui::{ui_button, ui_center, ui_column, ui_fullscreen, ui_global, ui_input, ui_row_parted};

use crate::{
  button, div, input, respo, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle,
  StatesTree,
};

use crate::alerts::{effect_fade, effect_focus, BUTTON_NAME};

const NEXT_TASK_NAME: &str = "_RESPO_PROMPT_NEXT_TASK";

#[derive(Debug, Clone, Default)]
pub struct PromptOptions {
  backdrop_style: RespoStyle,
  card_style: RespoStyle,
  text: Option<String>,
  button_text: Option<String>,
  initial_value: Option<String>,
  multilines: bool,
  input_style: RespoStyle,
  validator: Option<Validator>,
}

#[derive(Clone)]
struct Validator(Rc<dyn Fn(String) -> Result<bool, String>>);

impl Debug for Validator {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(&Validator ..)")
  }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct InputState {
  draft: String,
}

pub fn comp_prompt_modal<T, U, V>(
  states: StatesTree,
  options: PromptOptions,
  show: bool,
  on_submit: U,
  on_close: V,
) -> Result<RespoNode<T>, String>
where
  U: Fn(String, DispatchFn<T>) -> Result<(), String> + 'static,
  V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let cursor = states.path();
  let mut state: InputState = states.data.cast_or_default()?;
  if let Some(text) = &options.initial_value {
    state.draft = text.to_owned();
  }

  let state2 = state.clone();

  let read = Rc::new(on_submit);
  let close = Rc::new(on_close);
  let close2 = close.clone();

  Ok(
    RespoNode::new_component(
      "prompt-modal",
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
                  span()
                    .inner_text(options.text.unwrap_or_else(|| "Input your text:".to_owned()))
                    .to_owned(),
                  space(None, Some(8)),
                  div()
                    .children([input().class_list(&[ui_input()]).value(state.draft).to_owned()])
                    .to_owned(),
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
                          read(state2.draft.to_owned(), dispatch)?;
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
pub trait PromptPluginInterface<T, U>
where
  T: Debug + Clone + RespoAction,
  U: Fn(String, DispatchFn<T>) -> Result<(), String>,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show alert
  fn show<V>(&self, dispatch: DispatchFn<T>, next_task: V) -> Result<(), String>
  where
    V: Fn(String) -> Result<(), String> + 'static;
  /// to close alert
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  fn new(states: StatesTree, options: PromptOptions, on_submit: U) -> Result<Self, String>
  where
    Self: std::marker::Sized;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PromptPluginState {
  show: bool,
  text: Option<String>,
}

/// struct for PromptPlugin
#[derive(Debug, Clone)]
pub struct PromptPlugin<T, U>
where
  T: Clone + Debug,
  U: Fn(String, DispatchFn<T>) -> Result<(), String> + 'static,
{
  states: StatesTree,
  state: PromptPluginState,
  options: PromptOptions,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<String>,
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
    let cursor = self.cursor.clone();
    let cursor2 = self.cursor.clone();
    let state = self.state.to_owned();
    let state2 = self.state.to_owned();

    comp_prompt_modal(
      self.states.pick("plugin"),
      self.options.to_owned(),
      self.state.show,
      move |content, dispatch| {
        let d2 = dispatch.clone();
        on_submit(content.to_owned(), dispatch)?;
        let window = web_sys::window().expect("window");
        // dirty global variable
        let task = Reflect::get(&window, &JsValue::from_str(NEXT_TASK_NAME));
        if let Ok(f) = task {
          if f.is_function() {
            let f = f.dyn_into::<Function>().unwrap();
            let arr = Array::new();
            arr.push(&JsValue::from_str(&content.to_owned()));
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
        d2.run_state(&cursor, s)?;
        // clean up leaked closure
        let window = web_sys::window().expect("window");
        let _ = Reflect::set(&window, &JsValue::from_str(NEXT_TASK_NAME), &JsValue::NULL);
        Ok(())
      },
      move |dispatch| {
        let s = PromptPluginState {
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
      text: self.text.clone(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: PromptOptions, on_submit: U) -> Result<Self, String> {
    let cursor = states.path();
    let state: PromptPluginState = states.data.cast_or_default()?;

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
}
