// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use docset::SearchIndex;

mod router;
mod feeds;
mod docset;
mod debug;

#[tauri::command]
fn docsets() -> Vec<String> {
    let con = docset::open_my_db("./../docsets/hoge.db3").unwrap();
    let docsets = docset::docsets(&con);
    let mut names: Vec<String> = Vec::new();
    for d in docsets {
        names.push(d.name)
    }

    names
}

#[tokio::main]
async fn main() {
    let _ = debug::debug_print();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router::mount(), || ()))
        // .invoke_handler(tauri::generate_handler![docsets, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
