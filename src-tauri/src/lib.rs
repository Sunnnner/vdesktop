pub mod core;
pub mod utils;
use core::{api::{force_stop_vms, get_config, get_vms, is_exist_config, lock_vms, save_config, spice_viewer, start_vms, stop_vms, switch_server, unlock_vms}, config::ConfigManager, model::AppState};
use std::sync::Arc;
use tauri::Manager;



pub type Result<T> = std::result::Result<T, utils::error::Error>;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let home_dir = app.path().home_dir().unwrap();
            let path = home_dir.join(".config/config.yml");
            if !path.exists() {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            let config_dir = Arc::new(path.clone());
            let is_exist_config = config_dir.exists();
            
            let app_state = AppState {
                config_dir,
                is_exist_config,
                config_manager: ConfigManager::new(path.clone()),
            };
            app.manage(app_state);
            Ok(())
            })
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config, 
            is_exist_config,
            switch_server,
            get_vms,
            spice_viewer,
            start_vms,
            stop_vms,
            lock_vms,
            unlock_vms,
            force_stop_vms
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
