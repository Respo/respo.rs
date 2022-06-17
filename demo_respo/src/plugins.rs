use std::fmt::Debug;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoNode, StatesTree};

use respo::alerts::{AlertOptions, AlertPlugin, AlertPluginInterface};

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

  let p1 = Rc::new(alert_plugin);
  let p2 = p1.clone();

  let on_alert = move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
    util::log!("click {:?}", e);

    p1.show(dispatch, None)?;

    Ok(())
  };

  Ok(
    div()
      .children([
        div().children([span().inner_text("TODO").to_owned()]).to_owned(),
        div()
          .children([button().class(ui_button()).inner_text("Try Alert").on_click(on_alert).to_owned()])
          .to_owned(),
        p2.render()?,
      ])
      .to_owned(),
  )
}
