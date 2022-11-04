#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
// use dirs;
// use serde_yaml;

use serde::{Deserialize, Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn create_config(vdesk_config: String) {
//     // 创建$HOME/.config/vDesk.yaml文件如果不存在，如果存在则打开读取内容
//     let config_path = dirs::home_dir().unwrap().join(".config/vDesk.yaml");
//     let config_file = std::fs::File::open(config_path);
//     match config_file {
//         Ok(file) => {
//             let config: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();
//             println!("{:?}", config);
//         }
//         Err(err) => {
//             let mut map = BTreeMap::new();
//             map.insert("appid", "iRcTrJrDCEptuhdT");
//             map.insert("appsecret", "DYF9UoRjd6ayw9oE");
//             map.insert("url", "https://vdesk.knd.io");
//             map.insert("default-vm", "zhangxu");
//             let config = serde_yaml::to_string(&map).unwrap();
//             std::fs::write(config_path, config).unwrap();
//         }
//     }

// }

// Error
#[derive(Debug, Serialize, Deserialize)]
struct Error {
    code: i32,
    message: String,
}

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
fn read_yaml_file(path: String) -> Result<Config, Error> {
    let config_file = std::fs::File::open(path);
    match config_file {
        Ok(file) => {
            let config: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();
            let config = serde_yaml::from_value(config);
            match config {
                Ok(config) => Ok(config),
                Err(err) => Err(Error {
                    code: 1,
                    message: err.to_string(),
                }),
            }
        }
        Err(err) => Err(
            Error {
                code: 1,
                message: err.to_string(),
            }
        )
    }
}

#[tauri::command]
fn save_yaml_file(path: String, config: Config) -> Result<(), Error> {
    let home_dir = dirs::home_dir();
    match home_dir {
        Some(home_dir) => {
            let config_path = home_dir.join(".config/vdesk.yaml");
            let config = serde_yaml::to_string(&config);
            
            std::fs::write(config_path, config).unwrap();
            Ok(())
        }
        None => Err(Error {
            code: 1,
            message: "home dir not found".to_string(),
        }),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            read_yaml_file
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
