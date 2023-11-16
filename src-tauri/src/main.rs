#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
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

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_window::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(|app| {
      let win = tauri::WindowBuilder::new(
        app,
        "weread",
        tauri::WindowUrl::External("https://weread.qq.com".parse().unwrap()),
      )
      .initialization_script(include_str!("../inject/preload.js"))
      .initialization_script(include_str!("../inject/event.js"))
      .initialization_script(&inject_style(include_str!("../inject/style.css")))
      .build()?;
      let _ = win.set_title("微信读书");

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
