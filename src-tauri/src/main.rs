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

#[tauri::command]
fn search(word: &str) -> Vec<SearchIndex> {
    let con = docset::open_my_db("./../docsets/hoge.db3").unwrap();
    // let docsets = docset::search_docsets(&con, word);

    // let mut names: Vec<String> = Vec::new();
    // for d in docsets {
    //     names.push(d.name)
    // }

    let mut results: Vec<SearchIndex> = Vec::new();
    if word.is_empty() == false {
        let doc_con = docset::open_my_db("./../docsets/TypeScript.docset/Contents/Resources/docSet.dsidx").unwrap();
        let search_indices = docset::search_index(&doc_con, word);

        for index in search_indices {
            results.push(index)
        }
    }

    // names
    results
}

#[tokio::main]
async fn main() {
    let _ = debug::debug_print();

    tauri::Builder::default()
        .setup(|app| {
            let p = app.path_resolver().app_data_dir();
            println!("app data dir: {:?}", p.unwrap());
            
            let localp = app.path_resolver().app_local_data_dir();
            println!("local data dir: {:?}", localp.unwrap());

            let library = app.path_resolver().app_config_dir();
            println!("app config: {:?}", library);

            let appdir = app.path_resolver().app_dir();
            println!("app: {:?}", appdir.unwrap());
            Ok(())
        })
        .plugin(rspc::integrations::tauri::plugin(router::mount(), || ()))
        .invoke_handler(tauri::generate_handler![docsets, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
