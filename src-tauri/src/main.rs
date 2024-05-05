#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod tray;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}
use tauri_plugin_store::StoreBuilder;

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

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .plugin(tauri_plugin_store::Builder::default().build())
    .setup(|app| {
      #[cfg(all(desktop, not(test)))]
      {
        let handle = app.handle();
        tray::create_tray(handle)?;
      }
      let mut store = StoreBuilder::new(".settings.dat").build(app.handle().clone());
      store.load().expect("Failed to load store from disk");

      let font_family = store
        .get("font-family")
        .map(|t| t.to_string())
        .unwrap_or("霞鹜文楷".to_string());

      let css_snippet = store
        .get("css")
        .map(|t| t.to_string())
        .unwrap_or("".to_string());

      let css = format!("*:not(pre) {{ font-family: {font_family} !important; }}");
      // let width = 800.0;
      // let height = 600.0;
      // let win = tauri::window::WindowBuilder::new(app, "main")
      //   .inner_size(width, height)
      //   .decorations(false)
      //   .build()?;

      // let _webview1 = win.add_child(
      //   tauri::webview::WebviewBuilder::new("main1", WebviewUrl::App(Default::default()))
      //     .auto_resize(),
      //   LogicalPosition::new(0., 0.),
      //   LogicalSize::new(width, 30.),
      // )?;

      // let wechat = tauri::webview::WebviewBuilder::new(
      //   "main2",
      //   WebviewUrl::External("https://weread.qq.com".parse().unwrap()),
      // )
      // .auto_resize()
      // .initialization_script(include_str!("../inject/preload.js"))
      // .initialization_script(include_str!("../inject/event.js"))
      // .initialization_script(&inject_style(include_str!("../inject/style.css")));

      // let _webview2 = win.add_child(
      //   wechat,
      //   LogicalPosition::new(0., 30.),
      //   LogicalSize::new(width, height - 30.),
      // )?;

      let win = tauri::WebviewWindowBuilder::new(
        app,
        "weread",
        tauri::WebviewUrl::External("https://weread.qq.com".parse().unwrap()),
      )
      .title("微信读书")
      .visible(false)
      .initialization_script(include_str!("../inject/preload.js"))
      .initialization_script(include_str!("../inject/event.js"))
      .initialization_script(&inject_style(include_str!("../inject/style.css")))
      .initialization_script(&inject_style(&css_snippet))
      .initialization_script(&inject_style(&css))
      .build()?;

      win.show().unwrap();

      let id = win.listen("location", |event| {
        println!("got location with payload {:?}", event.payload());
      });

      win.unlisten(id);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
