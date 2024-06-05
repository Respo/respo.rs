use respo::ui::{ui_button_danger, ui_button_primary};
use respo::RespoEvent;
use respo::{space, ui::ui_row_parted, RespoStyle};

use respo::{button, div, span, ui::ui_button, util, DispatchFn, RespoNode, StatesTree};

use respo::dialog::{
  AlertOptions, AlertPlugin, AlertPluginInterface, ConfirmOptions, ConfirmPlugin, ConfirmPluginInterface, DrawerOptions, DrawerPlugin,
  DrawerPluginInterface, DrawerRenderer, ModalOptions, ModalPlugin, ModalPluginInterface, ModalRenderer, PromptOptions, PromptPlugin,
  PromptPluginInterface, PromptValidator,
};

use super::store::*;

pub fn comp_plugins_demo(states: &StatesTree) -> Result<RespoNode<ActionOp>, String> {
  // respo::util::log!("re-render");

  let alert_plugin = AlertPlugin::new(
    states.pick("info"),
    AlertOptions {
      // card_style: RespoStyle::default().background_color(CssColor::Blue).to_owned(),
      ..AlertOptions::default()
    },
    |_dispatch: DispatchFn<ActionOp>| {
      respo::util::log!("user has read the message");
      Ok(())
    },
  )?
  .share_with_ref();

  let on_alert = {
    let alert_plugin = alert_plugin.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      // alert_plugin.show(dispatch, Some("a mesasge for you".to_owned()))?;
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
    let confirm_plugin = confirm_plugin.to_owned();
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
      text: Some(String::from("Demo text(length 3~8)")),
      validator: Some(PromptValidator::new(|text| {
        if text.len() <= 1 {
          Err("too short".to_owned())
        } else if text.len() > 8 {
          Err("too long".to_owned())
        } else {
          Ok(())
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
    let prompt_plugin = prompt_plugin.to_owned();
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
      title: Some(String::from("Modal demo")),
      render: ModalRenderer::new(|close_modal: _| {
        let handler = move |_e: _, dispatch: DispatchFn<ActionOp>| {
          respo::util::log!("on modal handle");
          close_modal(dispatch)
        };
        Ok(
          div()
            .style(RespoStyle::default().padding(8.0).to_owned())
            .children([
              div().children([span().inner_text("content in custom modal").to_owned()]).to_owned(),
              div()
                .class(ui_row_parted())
                .children([span(), button().class(ui_button()).inner_text("close").on_click(handler).to_owned()])
                .to_owned(),
            ])
            .to_owned(),
        )
      }),
      ..ModalOptions::default()
    },
  )?
  .share_with_ref();

  let on_modal = {
    let modal_plugin = modal_plugin.to_owned();
    move |e, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      modal_plugin.show(dispatch)?;

      Ok(())
    }
  };

  // declare drawer

  let drawer_plugin = DrawerPlugin::new(
    states.pick("drawer"),
    DrawerOptions {
      title: Some(String::from("Modal demo")),
      render: DrawerRenderer::new(|close_drawer: _| {
        let handler = move |_e: _, dispatch: DispatchFn<ActionOp>| {
          respo::util::log!("on modal handle");
          close_drawer(dispatch)
        };
        Ok(
          div()
            .style(RespoStyle::default().padding(8.0).to_owned())
            .children([
              div()
                .children([span().inner_text("content in custom drawer").to_owned()])
                .to_owned(),
              div()
                .class(ui_row_parted())
                .children([span(), button().class(ui_button()).inner_text("close").on_click(handler).to_owned()])
                .to_owned(),
            ])
            .to_owned(),
        )
      }),
      ..DrawerOptions::default()
    },
  )?
  .share_with_ref();

  let on_drawer = {
    let drawer_plugin = drawer_plugin.to_owned();
    move |e: RespoEvent, dispatch: DispatchFn<_>| -> Result<(), String> {
      util::log!("click {:?}", e);

      drawer_plugin.show(dispatch)?;

      Ok(())
    }
  };

  Ok(
    div()
      .children([
        div().children([span().inner_text("Dialogs").to_owned()]).to_owned(),
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
            button()
              .class(ui_button_primary())
              .inner_text("Try Custom Modal")
              .on_click(on_modal)
              .to_owned(),
            space(Some(8), None),
            button()
              .class(ui_button_danger())
              .inner_text("Try Custom Drawer")
              .on_click(on_drawer)
              .to_owned(),
          ])
          .to_owned(),
        alert_plugin.render()?,
        confirm_plugin.render()?,
        prompt_plugin.render()?,
        modal_plugin.render()?,
        drawer_plugin.render()?,
      ])
      .to_owned(),
  )
}
