use std::fmt::Debug;
use std::rc::Rc;

use respo::space;
use serde::{Deserialize, Serialize};

use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoNode, StatesTree};

use respo::alerts::{
  AlertOptions, AlertPlugin, AlertPluginInterface, ConfirmOptions, ConfirmPlugin, ConfirmPluginInterface, PromptOptions, PromptPlugin,
  PromptPluginInterface, PromptValidator,
};

use super::store::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TaskState {
  draft: String,
  error: Option<String>,
}

pub fn comp_plugins_demo(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  let alert_plugin = AlertPlugin::new(states.pick("info"), AlertOptions::default(), |_dispatch: DispatchFn<ActionOp>| {
    respo::util::log!("on read");
    Ok(())
  })?;
  let alert_plugin = Rc::new(alert_plugin);
  let alert_plugin2 = alert_plugin.clone();

  let confirm_plugin = ConfirmPlugin::new(
    states.pick("confirm"),
    ConfirmOptions::default(),
    |_dispatch: DispatchFn<ActionOp>| {
      respo::util::log!("on confirm");
      Ok(())
    },
  )?;

  let confirm_plugin = Rc::new(confirm_plugin);
  let confirm_plugin3 = confirm_plugin.clone();
  let confirm_plugin2 = confirm_plugin;

  let options = PromptOptions {
    validator: Some(PromptValidator::new(|text| {
      if text.len() < 3 {
        Ok(())
      } else {
        Err("too long".to_owned())
      }
    })),
    multilines: true,
    ..Default::default()
  };

  let prompt_plugin = Rc::new(PromptPlugin::new(
    states.pick("prompt"),
    options,
    |content, _dispatch: DispatchFn<ActionOp>| {
      respo::util::log!("on prompt: {}", content);
      Ok(())
    },
  )?);
  let prompt_plugin2 = prompt_plugin.clone();

  let on_alert = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    alert_plugin.show(dispatch, None)?;

    Ok(())
  };

  let on_confirm = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    confirm_plugin2.show(dispatch, move || {
      respo::util::log!("do something on confirm");
      Ok(())
    })?;

    Ok(())
  };

  let on_prompt = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    prompt_plugin2.show(dispatch, move |content| {
      respo::util::log!("do something on prompt: {}", content);
      Ok(())
    })?;

    Ok(())
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
          ])
          .to_owned(),
        alert_plugin2.render()?,
        confirm_plugin3.render()?,
        prompt_plugin.render()?,
      ])
      .to_owned(),
  )
}
