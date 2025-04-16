// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use rspc_tauri; // Add this line

mod debug;
mod docset;
mod docset_downloader;
mod feeds;
mod router;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let _ = debug::debug_print(&app.handle()); // Pass AppHandle instead of App
            Ok(())
        })
        .plugin(rspc_tauri::plugin(router::mount(), |_| ())) // Change closure to take one argument
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
