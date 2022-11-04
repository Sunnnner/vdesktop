#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use dirs;
// use serde_yaml;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[serde_as]
#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(tag = "type", content = "message")]
pub enum Error {
    #[error("fs error: {0}")]
    Fs(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        std::io::Error,
    ),
    #[error("serde error: {0}")]
    Serde(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        serde_yaml::Error,
    ),
}



pub type Result<T> = std::result::Result<T, Error>;

// type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config{
    appid: String,
    appsecret: String,
    url: String,
    #[serde(rename = "default-vm")]
    default_vm: String,
}

#[tauri::command]
fn read_yaml_file(path: String) -> Result<Config> {
    let config_file = std::fs::File::open(path);
    match config_file {
        Ok(file) => {
            let config: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();
            let config = serde_yaml::from_value(config);
            match config {
                Ok(config) => Ok(config),
                Err(err) => Err(Error::Serde(err)),
            }
        }
        Err(err) => Err(Error::Fs(err)),
    }
}

#[tauri::command]
fn save_yaml_file(config: Config) -> Result<String> {
    let home_dir = dirs::home_dir()?;
    let config_dir = home_dir.join(".config");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    let config_path = config_dir.join("vdesk.yaml");
    if config_path.exists(){
        // 删除文件
        std::fs::remove_file(&config_path)?;
    }
    let config = serde_yaml::to_string(&config)?;
    let file = std::fs::write(config_path, config)?;
    Ok("ok".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            read_yaml_file,
            save_yaml_file
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
