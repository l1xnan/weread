// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::AppHandle;
use tauri::Wry;
use url::Url;

use serde_json::json;
use tauri::Manager;
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

  let _ = with_store(app.clone(), stores, path, |store| {
    let _ = store.insert("href".to_string(), json!(href));
    store.save()
  });

  format!("{}", href)
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
  tauri::Builder::default()
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      let _id = app.listen_global("location", |event| {
        let raw = event.payload().unwrap_or("");
        let payload: Result<Payload, _> = serde_json::from_str(raw);
        println!("got event-name with payload {:?}", payload);
      });

      let mut store = StoreBuilder::new(app.handle(), "settings.json".parse()?).build();

      let _ = app
        .handle()
        .plugin(tauri_plugin_store::Builder::default().build());
      let _ = store.load();

      let href = store
        .get("href")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or("https://weread.qq.com".to_string());

      let font_family = store
        .get("font-family")
        .map(|t| t.to_string())
        .unwrap_or("霞鹜文楷".to_string());
      let css = format!("*:not(pre) {{ font-family: {font_family} !important; }}");

      let css_snippet = store
        .get("css")
        .map(|t| t.to_string())
        .unwrap_or("".to_string());

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
      .initialization_script(&inject_style(&css))
      .initialization_script(&inject_style(&css_snippet))
      .build()?;

      let _ = window.show();
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![change_route])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
