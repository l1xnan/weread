// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_store::StoreBuilder;

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
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let mut store = StoreBuilder::new(app.handle(), ".settings.dat".parse()?).build();

            let _ = app.handle()
                .plugin(tauri_plugin_store::Builder::default().build());
            store.load();

            let font_family = store
                .get("font-family")
                .map(|t| t.to_string())
                .unwrap_or("霞鹜文楷".to_string());
            let css = format!("*:not(pre) {{ font-family: {font_family} !important; }}");

            let css_snippet = store
                .get("css")
                .map(|t| t.to_string())
                .unwrap_or("".to_string());

            let window = tauri::WindowBuilder::new(
                app,
                "external", /* the unique window label */
                tauri::WindowUrl::External("https://weread.qq.com".parse().unwrap()),
            )
            .title("微信读书")
            .visible(false)
            .initialization_script(include_str!("../inject/preload.js"))
            .initialization_script(include_str!("../inject/event.js"))
            .initialization_script(&inject_style(include_str!("../inject/style.css")))
            .initialization_script(&inject_style(&css))
            .initialization_script(&inject_style(&css_snippet))
            .build()?;

            window.show();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
