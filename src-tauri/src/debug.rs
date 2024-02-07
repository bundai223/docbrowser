use std::env;
use tauri::App;

pub fn debug_print(app: &App) -> std::io::Result<()> {
    println!("env::current_dir: {}", env::current_dir()?.display());
    println!("app_local_data_dir: {}", app.path_resolver().app_local_data_dir().unwrap().display());
    println!("app_data_dir: {}", app.path_resolver().app_data_dir().unwrap().display());
    println!("app_config_dir: {}", app.path_resolver().app_config_dir().unwrap().display());
    println!("app_cache_dir: {}", app.path_resolver().app_cache_dir().unwrap().display());

    Ok(())
}