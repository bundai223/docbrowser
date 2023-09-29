// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod router;
mod feeds;
mod docset;
mod docsetDownloader;
mod debug;

#[tokio::main]
async fn main() {
    let _ = debug::debug_print();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router::mount(), || ()))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
