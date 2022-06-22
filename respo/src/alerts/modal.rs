use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::alerts::{css_backdrop, css_card};
use crate::ui::{ui_center, ui_column, ui_fullscreen, ui_global};

use crate::{div, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree};

use crate::alerts::{effect_fade, effect_focus};

#[derive(Debug, Clone, Default)]
pub struct ModalOptions<T>
where
  T: Debug + Clone,
{
  pub backdrop_style: RespoStyle,
  pub card_style: RespoStyle,
  pub title: Option<String>,
  pub render: ModalRenderer<T>,
}

type ModalRendererFn<T> = dyn Fn(Rc<dyn Fn(DispatchFn<T>) -> Result<(), String>>) -> Result<RespoNode<T>, String>;

#[derive(Clone)]
pub struct ModalRenderer<T>(Rc<ModalRendererFn<T>>)
where
  T: Debug + Clone;

impl<T> Debug for ModalRenderer<T>
where
  T: Debug + Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "(&ModalRenderer ..)")
  }
}

impl<T> Default for ModalRenderer<T>
where
  T: Debug + Clone,
{
  fn default() -> Self {
    Self(Rc::new(|_close: _| Ok(div())))
  }
}

impl<T> ModalRenderer<T>
where
  T: Debug + Clone,
{
  pub fn new<V>(renderer: V) -> Self
  where
    V: Fn(Rc<dyn Fn(DispatchFn<T>) -> Result<(), String>>) -> Result<RespoNode<T>, String> + 'static,
  {
    Self(Rc::new(renderer))
  }

  pub fn run<V>(&self, close: V) -> Result<RespoNode<T>, String>
  where
    V: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  {
    (self.0)(Rc::new(close))
  }
}

pub fn comp_modal<T, U>(options: ModalOptions<T>, show: bool, on_close: U) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let close = Rc::new(on_close);
  let close2 = close.clone();

  Ok(
    RespoNode::new_component(
      "modal",
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
                .class(ui_column())
                .children([
                  span().inner_text(options.title.unwrap_or_else(|| "Modal".to_owned())).to_owned(),
                  space(None, Some(8)),
                  options.render.run(move |dispatch| -> Result<(), String> {
                    let close = close2.clone();
                    close(dispatch)?;
                    Ok(())
                  })?,
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
pub trait ModalPluginInterface<T>
where
  T: Debug + Clone + RespoAction,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show alert
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String>;
  /// to close alert
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  fn new(states: StatesTree, options: ModalOptions<T>) -> Result<Self, String>
  where
    Self: std::marker::Sized;
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct ModalPluginState {
  show: bool,
  text: Option<String>,
}

/// struct for AlertPlugin
#[derive(Debug, Clone)]
pub struct ModalPlugin<T>
where
  T: Clone + Debug,
{
  state: ModalPluginState,
  options: ModalOptions<T>,
  /// tracking content to display
  text: Option<String>,
  cursor: Vec<String>,
  phantom: PhantomData<T>,
}

impl<T> ModalPluginInterface<T> for ModalPlugin<T>
where
  T: Clone + Debug + RespoAction,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let cursor = self.cursor.clone();
    let state = self.state.to_owned();

    comp_modal(self.options.to_owned(), self.state.show, move |dispatch: DispatchFn<_>| {
      let s = ModalPluginState {
        show: false,
        text: state.text.to_owned(),
      };
      dispatch.run_state(&cursor, s)?;
      Ok(())
    })
  }
  fn show(&self, dispatch: DispatchFn<T>, text: Option<String>) -> Result<(), String> {
    let s = ModalPluginState {
      show: true,
      text: text.or_else(|| self.state.text.to_owned()),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = ModalPluginState {
      show: false,
      text: self.text.clone(),
    };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: ModalOptions<T>) -> Result<Self, String> {
    let cursor = states.path();
    let state: ModalPluginState = states.data.cast_or_default()?;

    let instance = Self {
      state,
      options,
      text: None,
      cursor,
      phantom: PhantomData,
    };

    Ok(instance)
  }
}
