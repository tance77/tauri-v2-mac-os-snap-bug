// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{ClickType, TrayIconBuilder};

use tauri::{WebviewWindow, WebviewWindowBuilder, WebviewUrl, App};



// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
fn open_main_window(handle: &App) -> WebviewWindow {
        let main_window = WebviewWindowBuilder::new(handle, "main", WebviewUrl::App("index.html".into()))
            .title("test")
            .resizable(true)
            .fullscreen(false)
            .min_inner_size(650.00, 500.00)
            .decorations(true)
            .inner_size(1280.00, 800.00)
            .build()
            .expect("Error building main window.");
        // Open devtools in debug mode
        main_window.open_devtools();
        return main_window;
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let quit = MenuItemBuilder::new("Quit MyTimeIn").id("quit").build(app).unwrap();
            let logs = MenuItemBuilder::new("Open Log Location").id("logs").build(app).unwrap();
            let show = MenuItemBuilder::new("Show Window").id("show").build(app).unwrap();
            let center = MenuItemBuilder::new("Center Window").id("center").build(app).unwrap();
            let about = MenuItemBuilder::new("About MyTimeIn").id("about").build(app).unwrap();

            let menu = MenuBuilder::new(app).items(&[&show, &center, &logs, &about, &quit]).build().unwrap();
            let _window = open_main_window(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
