// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use docset::SearchIndex;

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
fn search(word: &str) -> Vec<SearchIndex> {
    let con = docset::open_my_db("./../docsets/hoge.db3").unwrap();
    let docsets = docset::search_docsets(&con, word);

    // let mut names: Vec<String> = Vec::new();
    // for d in docsets {
    //     names.push(d.name)
    // }

    let mut results: Vec<SearchIndex> = Vec::new();
    if word.is_empty() == false {
        let doccon = docset::open_my_db("./../docsets/TypeScript.docset/Contents/Resources/docSet.dsidx").unwrap();
        let search_indices = docset::search_index(&doccon, word);

        for index in search_indices {
            results.push(index)
        }
    }

    // names
    results
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![docsets, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
