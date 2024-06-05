use std::fmt::Debug;

use std::marker::PhantomData;
use std::rc::Rc;

use crate::dialog::{css_backdrop, css_modal_card};
use crate::ui::{column, ui_center, ui_fullscreen, ui_global};

use crate::{div, space, span, CssLineHeight, CssPosition, DispatchFn, RespoAction, RespoEvent, RespoNode, RespoStyle, StatesTree};

use crate::dialog::effect_modal_fade;

use super::comp_esc_listener;

/// The options for custom modal.
#[derive(Debug, Clone, Default)]
pub struct ModalOptions<T>
where
  T: Debug + Clone,
{
  /// inline style for backdrop
  pub backdrop_style: RespoStyle,
  /// inline style for card
  pub card_style: RespoStyle,
  /// title of the modal, defaults to `Modal`
  pub title: Option<String>,
  /// render body
  pub render: ModalRenderer<T>,
}

type ModalRendererFn<T> = dyn Fn(Rc<dyn Fn(DispatchFn<T>) -> Result<(), String>>) -> Result<RespoNode<T>, String>;

/// wraps render function
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

fn comp_modal<T, U>(options: ModalOptions<T>, show: bool, on_close: U) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  let close = Rc::new(on_close);

  Ok(
    RespoNode::new_component(
      "modal",
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
                close(dispatch)?;
                Ok(())
              }
            })
            .children([
              div()
                .class_list(&[column(), ui_global(), css_modal_card()])
                .style(RespoStyle::default().padding(0.0).line_height(CssLineHeight::Px(32.0)).to_owned())
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
                  .class(column())
                  .children([
                    div()
                      .class(ui_center())
                      .children([span().inner_text(options.title.unwrap_or_else(|| "Modal".to_owned())).to_owned()])
                      .to_owned(),
                    space(None, Some(8)),
                    {
                      let close = close.to_owned();
                      options.render.run(move |dispatch| -> Result<(), String> {
                        close(dispatch)?;
                        Ok(())
                      })?
                    },
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

/// provides the interfaces to component of custom modal dialog
pub trait ModalPluginInterface<T>
where
  T: Debug + Clone + RespoAction,
{
  /// renders UI
  fn render(&self) -> Result<RespoNode<T>, String>
  where
    T: Clone + Debug;
  /// to show modal
  fn show(&self, dispatch: DispatchFn<T>) -> Result<(), String>;
  /// to close modal
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String>;

  fn new(states: StatesTree, options: ModalOptions<T>) -> Result<Self, String>
  where
    Self: std::marker::Sized;

  /// share it with `Rc`
  fn share_with_ref(&self) -> Rc<Self>;
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ModalPluginState {
  show: bool,
}

/// a modal that you can render you down card body
#[derive(Debug, Clone)]
pub struct ModalPlugin<T>
where
  T: Clone + Debug,
{
  state: Rc<ModalPluginState>,
  options: ModalOptions<T>,
  /// tracking content to display
  cursor: Vec<Rc<str>>,
  phantom: PhantomData<T>,
}

impl<T> ModalPluginInterface<T> for ModalPlugin<T>
where
  T: Clone + Debug + RespoAction,
{
  fn render(&self) -> Result<RespoNode<T>, String> {
    let cursor = self.cursor.to_owned();

    comp_modal(self.options.to_owned(), self.state.show, move |dispatch: DispatchFn<_>| {
      let s = ModalPluginState { show: false };
      dispatch.run_state(&cursor, s)?;
      Ok(())
    })
  }
  fn show(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = ModalPluginState { show: true };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }
  fn close(&self, dispatch: DispatchFn<T>) -> Result<(), String> {
    let s = ModalPluginState { show: false };
    dispatch.run_state(&self.cursor, s)?;
    Ok(())
  }

  fn new(states: StatesTree, options: ModalOptions<T>) -> Result<Self, String> {
    let cursor = states.path();
    let state = states.data.cast_or_default::<ModalPluginState>()?;

    let instance = Self {
      state,
      options,
      cursor,
      phantom: PhantomData,
    };

    Ok(instance)
  }

  fn share_with_ref(&self) -> Rc<Self> {
    Rc::new(self.to_owned())
  }
}
