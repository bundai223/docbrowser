use std::env;
use tauri::{AppHandle, Manager}; // Add Manager to imports

pub fn debug_print(app_handle: &AppHandle) -> std::io::Result<()> { // Change argument name and type
    println!("env::current_dir: {}", env::current_dir()?.display());
    println!(
        "app_local_data_dir: {}",
        // Use app_handle.path() instead of app.path_resolver()
        app_handle.path().app_local_data_dir().unwrap().display()
    );
    println!(
        "app_data_dir: {}",
        app_handle.path().app_data_dir().unwrap().display()
    );
    println!(
        "app_config_dir: {}",
        app_handle.path().app_config_dir().unwrap().display()
    );
    println!(
        "app_cache_dir: {}",
        app_handle.path().app_cache_dir().unwrap().display()
    );

    Ok(())
}
