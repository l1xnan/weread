#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

const INIT_SCRIPT: &str = r#"
    if (window.location.origin === 'https://weread.qq.com') {
      console.log("hello world from js init script");
  
      window.__MY_CUSTOM_PROPERTY__ = { foo: 'bar' };
    }
  "#;

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
    .setup(|app| {
      let win = tauri::WindowBuilder::new(
        app,
        "weread",
        tauri::WindowUrl::External("https://weread.qq.com".parse().unwrap()),
      )
      .initialization_script(INIT_SCRIPT)
      .initialization_script(&inject_style(include_str!("../inject/style.css")))
      .build()?;
      let _ = win.set_title("微信读书");
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
