// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value as JsonValue;
use tauri::AppHandle;
use tauri::SystemTrayMenuItem;
use tauri::Wry;
use url::Url;

use serde_json::json;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_store::with_store;
use tauri_plugin_store::StoreBuilder;
use tauri_plugin_store::StoreCollection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn change_route(app: AppHandle, href: &str) -> String {
  let stores = app.state::<StoreCollection<Wry>>();
  let path = app
    .path_resolver()
    .app_data_dir()
    .unwrap()
    .join("settings.json");

  let mut store = StoreBuilder::new(app, "settings.json".parse().unwrap()).build();
  let _ = store.load();

  let _ = store.insert("href".to_string(), json!(href));
  store.save();

  // let _ = with_store(app, stores, path, |store| {
  //   let _ = store.insert("href".to_string(), json!(href));
  //   store.save()
  // });

  format!("{}", href)
}

#[tauri::command]
async fn create_setting(handle: AppHandle) {
  if let Some(win) = handle.get_window("setting") {
    let _ = win.show();
  } else {
    let _webview = tauri::WindowBuilder::new(
      &handle,
      "setting", /* the unique window label */
      tauri::WindowUrl::default(),
    )
    .title("设置")
    .build()
    .unwrap();
  }
}

#[tauri::command]
fn get_store(handle: AppHandle, key: &str) -> Option<String> {
  let mut store = StoreBuilder::new(handle, "settings.json".parse().unwrap()).build();
  let _ = store.load();
  store.get(key).map(|t| t.to_string())
}

fn inject_style(css: &str) -> String {
  format!(
    r#"
        document.addEventListener('DOMContentLoaded', _event => {{
            const weReadStyle = `\{}`;
            const weReadStyleElement = document.createElement('style');
            weReadStyleElement.innerHTML = weReadStyle;
            document.head.appendChild(weReadStyleElement);
            console.log("inject style");
        }})
        "#,
    css
  )
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Payload {
  href: String,
}

static BASE_URL: &str = "https://weread.qq.com";

fn main() {
  let setting = CustomMenuItem::new("setting".to_string(), "设置");
  let quit = CustomMenuItem::new("quit".to_string(), "退出");
  let hide = CustomMenuItem::new("hide".to_string(), "隐藏");
  let tray_menu = SystemTrayMenu::new()
    .add_item(setting)
    .add_item(quit)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(hide);

  let system_tray = SystemTray::new().with_menu(tray_menu);

  tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_store::Builder::default().build())
    // .on_window_event(|event| match event.event() {
    //   tauri::WindowEvent::CloseRequested { api, .. } => {
    //     let win = event.window();
    //     if win.label() == "setting" {
    //       win.hide().unwrap();
    //       api.prevent_close();
    //     }
    //   }
    //   _ => {}
    // })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // get a handle to the clicked menu item
        // note that `tray_handle` can be called anywhere,
        // just get an `AppHandle` instance with `app.handle()` on the setup hook
        // and move it to another function or thread
        let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "hide" => {
            let window = app.get_window("weread").unwrap();
            window.hide().unwrap();
            // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
            item_handle.set_title("Show").unwrap();
          }
          "setting" => {
            if let Some(win) = app.get_window("setting") {
              let _ = win.show();
            } else {
              let _webview = tauri::WindowBuilder::new(
                app,
                "setting", /* the unique window label */
                tauri::WindowUrl::default(),
              )
              .title("设置")
              .build()
              .unwrap();
            }
          }
          _ => {}
        }
      }
      _ => {}
    })
    .setup(|app| {
      let _id = app.listen_global("location", |event| {
        let raw = event.payload().unwrap_or("");
        let payload: Result<Payload, _> = serde_json::from_str(raw);
        println!("got event-name with payload {:?}", payload);
      });

      let mut store = StoreBuilder::new(app.handle(), "settings.json".parse()?).build();
      let _ = store.load();

      // let _ = app
      //   .handle()
      //   .plugin(tauri_plugin_store::Builder::default().build());

      let href = store
        .get("href")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or("https://weread.qq.com".to_string());

      let url = Url::parse(&href).unwrap_or(BASE_URL.parse().unwrap());

      let window = tauri::WindowBuilder::new(
        app,
        "weread", /* the unique window label */
        tauri::WindowUrl::External(url),
      )
      .title("微信读书")
      .visible(false)
      .initialization_script(include_str!("../inject/preload.js"))
      .initialization_script(include_str!("../inject/event.js"))
      .initialization_script(&inject_style(include_str!("../inject/style.css")))
      .build()?;

      let _ = window.show();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      change_route,
      create_setting,
      get_store
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
