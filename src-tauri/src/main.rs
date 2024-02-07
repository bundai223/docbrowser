// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

mod router;
mod feeds;
mod docset;
mod docset_downloader;
mod debug;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let _ = debug::debug_print(&app);
            Ok(())
        })
        .plugin(rspc::integrations::tauri::plugin(router::mount(), || ()))
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
