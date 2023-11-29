// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod router;
mod feeds;
mod docset;
mod debug;

#[tokio::main]
async fn main() {
    let _ = debug::debug_print();

    tauri::Builder::default()
        .setup(|app| {
            let p = app.path_resolver().app_data_dir();
            println!("app data dir: {:?}", p.unwrap());
            
            let local_path = app.path_resolver().app_local_data_dir();
            println!("local data dir: {:?}", local_path.unwrap());

            let library = app.path_resolver().app_config_dir();
            println!("app config: {:?}", library);

            let app_dir = app.path_resolver().app_dir();
            println!("app: {:?}", app_dir.unwrap());
            Ok(())
        })
        .plugin(rspc::integrations::tauri::plugin(router::mount(), || ()))
        // .invoke_handler(tauri::generate_handler![docsets, search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
