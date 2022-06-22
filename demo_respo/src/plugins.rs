use std::fmt::Debug;

use respo::space;
use serde::{Deserialize, Serialize};

use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoNode, StatesTree};

use respo::dialog::{
  AlertOptions, AlertPlugin, AlertPluginInterface, ConfirmOptions, ConfirmPlugin, ConfirmPluginInterface, ModalOptions, ModalPlugin,
  ModalPluginInterface, ModalRenderer, PromptOptions, PromptPlugin, PromptPluginInterface, PromptValidator,
};

use super::store::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TaskState {
  draft: String,
  error: Option<String>,
}

pub fn comp_plugins_demo(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  // respo::util::log!("re-render");

  let alert_plugin = AlertPlugin::new(states.pick("info"), AlertOptions::default(), |_dispatch: DispatchFn<ActionOp>| {
    respo::util::log!("on read");
    Ok(())
  })?
  .share_with_ref();

  let on_alert = {
    let alert_plugin = alert_plugin.clone();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      alert_plugin.show(dispatch, None)?;

      Ok(())
    }
  };

  let confirm_plugin = ConfirmPlugin::new(
    states.pick("confirm"),
    ConfirmOptions::default(),
    |_dispatch: DispatchFn<ActionOp>| {
      respo::util::log!("on confirm");
      Ok(())
    },
  )?
  .share_with_ref();

  let on_confirm = {
    let confirm_plugin = confirm_plugin.clone();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      confirm_plugin.show(dispatch, move || {
        respo::util::log!("do something on confirm");
        Ok(())
      })?;

      Ok(())
    }
  };

  let prompt_plugin = PromptPlugin::new(
    states.pick("prompt"),
    PromptOptions {
      validator: Some(PromptValidator::new(|text| {
        if text.len() < 3 {
          Ok(())
        } else {
          Err("too long".to_owned())
        }
      })),
      multilines: true,
      ..Default::default()
    },
    |content, _dispatch: DispatchFn<ActionOp>| {
      respo::util::log!("on prompt: {}", content);
      Ok(())
    },
  )?
  .share_with_ref();

  let on_prompt = {
    let prompt_plugin = prompt_plugin.clone();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      prompt_plugin.show(dispatch, move |content| {
        respo::util::log!("do something on prompt: {}", content);
        Ok(())
      })?;

      Ok(())
    }
  };

  // declare modal

  let modal_plugin = ModalPlugin::new(
    states.pick("modal"),
    ModalOptions {
      render: ModalRenderer::new(|close_modal: _| {
        let handler = move |_e: _, dispatch: DispatchFn<ActionOp>| {
          respo::util::log!("on modal handle");
          close_modal(dispatch)
        };
        Ok(
          div()
            .children([
              span().inner_text("TODO").to_owned(),
              button().class(ui_button()).inner_text("close").on_click(handler).to_owned(),
            ])
            .to_owned(),
        )
      }),
      ..ModalOptions::default()
    },
  )?
  .share_with_ref();

  let on_modal = {
    let modal_plugin = modal_plugin.clone();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      modal_plugin.show(dispatch)?;

      Ok(())
    }
  };

  Ok(
    div()
      .children([
        div().children([span().inner_text("TODO").to_owned()]).to_owned(),
        div()
          .children([
            button().class(ui_button()).inner_text("Try Alert").on_click(on_alert).to_owned(),
            space(Some(8), None),
            button()
              .class(ui_button())
              .inner_text("Try Confirm")
              .on_click(on_confirm)
              .to_owned(),
            space(Some(8), None),
            button().class(ui_button()).inner_text("Try Prompt").on_click(on_prompt).to_owned(),
            space(Some(8), None),
            button().class(ui_button()).inner_text("Try Modal").on_click(on_modal).to_owned(),
          ])
          .to_owned(),
        alert_plugin.render()?,
        confirm_plugin.render()?,
        prompt_plugin.render()?,
        modal_plugin.render()?,
      ])
      .to_owned(),
  )
}
