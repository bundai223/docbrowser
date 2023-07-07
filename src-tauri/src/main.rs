// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod feeds;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn docsets() -> [&'static str; 2] {
    ["docset a", "docset b" ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, docsets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
