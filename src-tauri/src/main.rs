// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod feeds;
mod docset;

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

#[tauri::command]
fn search(word: &str) -> Vec<String> {
    let con = docset::open_my_db("./../docsets/hoge.db3").unwrap();
    let docsets = docset::search_docsets(&con, word);

    let mut names: Vec<String> = Vec::new();
    for d in docsets {
        names.push(d.name)
    }

    names
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![docsets, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
