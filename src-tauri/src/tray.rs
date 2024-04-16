// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use tauri::{
  menu::{Menu, MenuItem},
  tray::{ClickType, TrayIconBuilder},
  Manager, Runtime, WebviewUrl, WebviewWindowBuilder,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
  let w_setting = MenuItem::with_id(app, "setting", "设置", true, None::<&str>)?;
  let w_quit = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

  let menus = Menu::with_items(app, &[&w_setting, &w_quit])?;

  let _ = TrayIconBuilder::with_id("tray-1")
    .tooltip("Tauri")
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menus)
    .menu_on_left_click(false)
    .on_menu_event(move |app, event| match event.id.as_ref() {
      "quit" => {
        app.exit(0);
      }
      "setting" => {
        let _webview = WebviewWindowBuilder::new(app, "main-setting", WebviewUrl::default())
          .title("设置")
          .build()
          .unwrap();
      }
      _ => {}
    })
    .on_tray_icon_event(|tray, event| {
      if event.click_type == ClickType::Left {
        let app = tray.app_handle();
        if let Some(window) = app.get_webview_window("main") {
          let _ = window.show();
          let _ = window.set_focus();
        }
      }
    })
    .build(app);

  Ok(())
}
