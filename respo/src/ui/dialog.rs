//! module to provide popup dialogs.

mod alert;
mod confirm;
mod drawer;
mod modal;
mod prompt;

use js_sys::Reflect;
use std::fmt::Debug;
use std::rc::Rc;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlElement, KeyboardEvent, KeyboardEventInit, Node};

use crate::component::effect::RespoEffect;
use crate::node::css::{CssColor, CssDisplay, CssOverflow, CssPosition, CssSize, RespoStyle};
use crate::node::{DispatchFn, RespoEvent, RespoNode};
use crate::{app, input, static_styles, util, RespoComponent};

pub(crate) const BUTTON_NAME: &str = "dialog-button";

pub use alert::{AlertOptions, AlertPlugin, AlertPluginInterface};
pub use confirm::{ConfirmOptions, ConfirmPlugin, ConfirmPluginInterface};
pub use drawer::{DrawerOptions, DrawerPlugin, DrawerPluginInterface, DrawerRenderer};
pub use modal::{ModalOptions, ModalPlugin, ModalPluginInterface, ModalRenderer};
pub use prompt::{PromptOptions, PromptPlugin, PromptPluginInterface, PromptValidator};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectFocus {
  show: bool,
}

impl RespoEffect for EffectFocus {
  fn updated(&self, el: &Node) -> Result<(), String> {
    let show: bool = self.show;
    if show {
      focus_element(el, BUTTON_NAME)?;
    }

    Ok(())
  }
}

fn focus_element(el: &Node, name: &str) -> Result<(), String> {
  match el.dyn_ref::<Element>().unwrap().query_selector(&format!(".{}", name)) {
    Ok(Some(element)) => {
      match element.dyn_ref::<HtmlElement>() {
        Some(el) => el.focus().unwrap(),
        None => {
          app::util::log!("focus_element: {} is not an HTML element", name);
        }
      };
    }
    Ok(None) => {
      app::util::warn_log!("Attempted to focus on element '{}', but it was not found in the DOM.", name);
    }
    Err(e) => {
      app::util::warn_log!("focus_element: {} not found: {:?}", name, e);
    }
  }
  Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectModalFade {
  show: bool,
}

impl RespoEffect for EffectModalFade {
  fn before_update(&self, el: &Node) -> Result<(), String> {
    let show: bool = self.show;

    if !show {
      // when closing, fade out the cloned element
      match el.first_child() {
        Some(target) => {
          let d = target.clone_node_with_deep(true).unwrap();
          let cloned = Rc::new(d.dyn_ref::<HtmlElement>().unwrap().to_owned()); // outlive
          let document = el.owner_document().unwrap();
          document.body().unwrap().append_child(&cloned).unwrap();
          // setTimeout
          let window = web_sys::window().unwrap();
          let immediate_call: Closure<dyn FnMut()> = Closure::once({
            let cloned = cloned.to_owned();
            move || {
              let style = cloned.style();
              style.set_property("opacity", "0").unwrap();
              let card = cloned.first_child().unwrap();
              let card_style = card.dyn_ref::<HtmlElement>().unwrap().style();
              card_style.set_property("transition-duration", "240ms").unwrap();
              card_style.set_property("transform", "scale(0.94) translate(0px,-20px)").unwrap();
            }
          });
          window
            .set_timeout_with_callback_and_timeout_and_arguments_0(immediate_call.as_ref().unchecked_ref(), 10)
            .unwrap();
          immediate_call.forget();
          let delay_call: Closure<dyn FnMut()> = Closure::once(move || {
            cloned.remove();
          });
          window
            .set_timeout_with_callback_and_timeout_and_arguments_0(delay_call.as_ref().unchecked_ref(), 250)
            .unwrap();
          delay_call.forget();
        }
        None => {
          util::log!("content not found");
        }
      }
    }
    Ok(())
  }

  fn updated(&self, el: &Node) -> Result<(), String> {
    let show: bool = self.show;
    if show {
      // when opening, fade in the cloned element
      let target = el.first_child().unwrap();
      let style = target.dyn_ref::<HtmlElement>().unwrap().style();
      let card_style = target.first_child().unwrap().dyn_ref::<HtmlElement>().unwrap().style();
      style.set_property("opacity", "0").unwrap();
      card_style.set_property("transform", "scale(0.94) translate(0px,-12px)").unwrap();
      let call = Closure::once(move || {
        style.set_property("transition-duration", "240ms").unwrap();
        card_style.set_property("transition-duration", "240ms").unwrap();
        style.set_property("opacity", "1").unwrap();
        card_style.set_property("transform", "scale(1) translate(0px,0px)").unwrap();
      });
      let window = web_sys::window().unwrap();
      window
        .set_timeout_with_callback_and_timeout_and_arguments_0(call.as_ref().unchecked_ref(), 10)
        .unwrap();
      call.forget();
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EffectDrawerFade {
  show: bool,
}

impl RespoEffect for EffectDrawerFade {
  fn before_update(&self, el: &Node) -> Result<(), String> {
    let show = self.show;

    if !show {
      // when closing, fade out the cloned element
      match el.first_child() {
        Some(target) => {
          let d = target.clone_node_with_deep(true).unwrap();
          let cloned = Rc::new(d.dyn_ref::<HtmlElement>().unwrap().to_owned()); // outlive
          let document = el.owner_document().unwrap();
          document.body().unwrap().append_child(&cloned).unwrap();
          // setTimeout
          let window = web_sys::window().unwrap();
          let immediate_call: Closure<dyn FnMut()> = Closure::once({
            let cloned = cloned.to_owned();
            move || {
              let style = cloned.style();
              style.set_property("opacity", "0").unwrap();
              let card = cloned.first_child().unwrap();
              let card_style = card.dyn_ref::<HtmlElement>().unwrap().style();
              card_style.set_property("transition-duration", "240ms").unwrap();
              card_style.set_property("transform", "translate(100%,0px)").unwrap();
            }
          });
          window
            .set_timeout_with_callback_and_timeout_and_arguments_0(immediate_call.as_ref().unchecked_ref(), 10)
            .unwrap();
          immediate_call.forget();
          let delay_call: Closure<dyn FnMut()> = Closure::once(move || {
            cloned.remove();
          });
          window
            .set_timeout_with_callback_and_timeout_and_arguments_0(delay_call.as_ref().unchecked_ref(), 250)
            .unwrap();
          delay_call.forget();
        }
        None => {
          app::util::log!("content not found");
        }
      }
    }

    Ok(())
  }

  fn updated(&self, el: &Node) -> Result<(), String> {
    let show = self.show;

    if show {
      // when opening, fade in the cloned element
      let target = el.first_child().unwrap();
      let style = target.dyn_ref::<HtmlElement>().unwrap().style();
      let card_style = target.first_child().unwrap().dyn_ref::<HtmlElement>().unwrap().style();
      style.set_property("opacity", "0").unwrap();
      card_style.set_property("transform", "translate(100%, 0px)").unwrap();
      let call = Closure::once(move || {
        style.set_property("transition-duration", "240ms").unwrap();
        card_style.set_property("transition-duration", "240ms").unwrap();
        style.set_property("opacity", "1").unwrap();
        card_style.set_property("transform", "translate(0px,0px)").unwrap();
      });
      let window = web_sys::window().unwrap();
      window
        .set_timeout_with_callback_and_timeout_and_arguments_0(call.as_ref().unchecked_ref(), 10)
        .unwrap();
      call.forget();
    }

    Ok(())
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EffectModalClose {}

impl RespoEffect for EffectModalClose {
  fn mounted(&self, el: &Node) -> Result<(), String> {
    let el = Rc::new(el.to_owned());
    let window = web_sys::window().unwrap();
    let listener = Closure::wrap(Box::new({
      let el = el.to_owned();
      move |event: web_sys::KeyboardEvent| {
        let mut init_dict: KeyboardEventInit = KeyboardEventInit::new();
        init_dict
          .key(&event.key())
          .code(&event.code())
          .char_code(event.char_code())
          .view(event.view().as_ref())
          .location(event.location())
          .key_code(event.key_code());
        let new_event = KeyboardEvent::new_with_keyboard_event_init_dict(&event.type_(), &init_dict)
          .expect("Failed to create new KeyboardEvent from init dict");

        el.dispatch_event(&new_event).unwrap();
      }
    }) as Box<dyn FnMut(_)>);
    window
      .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
      .unwrap();
    let _ = Reflect::set(&el, &JsValue::from_str(TEMP_LISTENER), listener.as_ref().unchecked_ref());
    listener.forget();
    Ok(())
  }

  fn before_unmount(&self, el: &Node) -> Result<(), String> {
    let el = Rc::new(el.to_owned());
    let listener = Reflect::get(&el, &JsValue::from_str(TEMP_LISTENER)).unwrap();
    let window = web_sys::window().unwrap();
    window
      .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
      .unwrap();
    let _ = Reflect::set(&el, &JsValue::from_str(TEMP_LISTENER), &JsValue::NULL);

    Ok(())
  }
}

/// put listener on the element, directly on the element
const TEMP_LISTENER: &str = "temp_listener";

/// handle global keydown event
fn comp_esc_listener<T, U>(_show: bool, on_close: Rc<U>) -> Result<RespoNode<T>, String>
where
  U: Fn(DispatchFn<T>) -> Result<(), String> + 'static,
  T: Clone + Debug,
{
  Ok(
    RespoComponent::named(
      "esc-listener",
      input()
        .style(RespoStyle::default().display(CssDisplay::None))
        .on_keydown(move |e, dispatch| -> Result<(), String> {
          if let RespoEvent::Keyboard { key, .. } = e {
            if key == "Escape" {
              on_close(dispatch)?;
            }
          }
          Ok(())
        }),
    )
    .effect(EffectModalClose {})
    .to_node()
    .rc(),
  )
}

static_styles!(
  css_backdrop,
  (
    "&",
    RespoStyle::default()
      .background_color(CssColor::Hsla(0.0, 30.0, 10.0, 0.6))
      .position(CssPosition::Fixed)
      .z_index(999)
  )
);

static_styles!(
  css_modal_card,
  (
    "&",
    RespoStyle::default()
      .background_color(CssColor::Hsl(0, 0, 100))
      .max_width(CssSize::Px(600.0))
      .width(CssSize::Percent(100.))
      .max_height(CssSize::Vh(80.0))
      .overflow(CssOverflow::Auto)
      .border_radius(3.0)
      .color(CssColor::Hsl(0, 0, 0))
      .insert("margin", "auto".to_owned())
      .padding(16.0)
  )
);

static_styles!(
  css_drawer_card,
  (
    "&",
    RespoStyle::default()
      .background_color(CssColor::Hsl(0, 0, 100))
      .max_width(CssSize::Vw(50.0))
      .width(CssSize::Px(400.))
      .height(CssSize::Vh(100.0))
      .overflow(CssOverflow::Auto)
      .color(CssColor::Hsl(0, 0, 0))
      .top(CssSize::Px(0.))
      .right(CssSize::Px(0.))
      .bottom(CssSize::Px(0.))
      .position(CssPosition::Absolute)
      .box_shadow(-2., 0., 12., 0., CssColor::Hsla(0., 0., 0., 0.2))
      .transform_property("transform, opacity".to_owned())
  )
);

static_styles!(
  css_button,
  (
    "&",
    RespoStyle::default()
      .border_radius(4.0)
      .background_color(CssColor::White)
      .border_color(CssColor::Hsl(0, 0, 0))
  )
);
