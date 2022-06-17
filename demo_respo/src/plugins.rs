use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

use respo::space;
use serde::{Deserialize, Serialize};

use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoNode, StatesTree};

use respo::alerts::{AlertOptions, AlertPlugin, AlertPluginInterface, ConfirmOptions, ConfirmPlugin, ConfirmPluginInterface};

use super::store::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TaskState {
  draft: String,
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

  let confirm_plugin = Rc::new(RefCell::new(confirm_plugin));
  let confirm_plugin3 = confirm_plugin.clone();
  let confirm_plugin2 = confirm_plugin;

  let on_alert = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    alert_plugin.show(dispatch, None)?;

    Ok(())
  };

  let on_confirm = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    let mut confirm_plugin2 = confirm_plugin2.borrow_mut();
    confirm_plugin2.show(dispatch, move || {
      respo::util::log!("do something on confirm");
      Ok(())
    })?;

    Ok(())
  };

  // borrow again since mutable borrow happen
  let confirm_plugin3 = confirm_plugin3.borrow();

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
          ])
          .to_owned(),
        alert_plugin2.render()?,
        confirm_plugin3.render()?,
      ])
      .to_owned(),
  )
}
