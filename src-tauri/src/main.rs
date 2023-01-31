#![cfg_attr(
    all(target_os = "windows"),
    windows_subsystem = "windows"
)]
// use dirs;
// use serde_yaml;

use std::string::FromUtf8Error;

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
    #[error("FromUtf8 error: {0}")]
    Tauri(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        FromUtf8Error,
    ),
    #[error("Tauri Api error: {0}")]
    TauriApiError(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        tauri::api::Error,
    ),
}

pub type Result<T> = std::result::Result<T, Error>;

// type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config {
    appid: String,
    appsecret: String,
    url: String,
    #[serde(rename = "default-vm")]
    default_vm: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct VmList {
    vms: String,
    // string or null
    locked: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SuccessMessage {
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CodeMessage {
    message: String,
    code: String,
}

#[tauri::command]
fn read_yaml_file(path: String) -> Result<Config> {
    let config_file = std::fs::File::open(path)?;
    let config: serde_yaml::Value = serde_yaml::from_reader(config_file).unwrap();
    let config = serde_yaml::from_value(config)?;
    Ok(config)
}

#[tauri::command]
fn save_yaml_file(config: Config) -> Result<SuccessMessage> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    let config_path = config_dir.join("vdesk.yaml");
    if config_path.exists() {
        // 删除文件
        std::fs::remove_file(&config_path)?;
    }
    let config = serde_yaml::to_string(&config)?;
    std::fs::write(config_path, config)?;
    Ok(SuccessMessage {
        message: "保存成功".to_string(),
    })
}

// 列出所有的虚拟机
#[tauri::command]
fn list_vm() -> Result<Vec<VmList>> {
    let mut vms = Vec::new();
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
                .arg(r#"vd list"#)
                .output()?
    } else {
        std::process::Command::new("sh")
                .arg(r#"vd list"#)
                .output()?
    };
    let stdout = String::from_utf8(output.stdout)?;
    for line in stdout.lines() {
        if line.contains("!") {
            let vm = line.split("!!").collect::<Vec<&str>>();
            let name = vm[0].trim().split('-').collect::<Vec<&str>>()[1]
                .split("(")
                .collect::<Vec<&str>>()[0];
            let locked = vm[1].trim();
            vms.push(VmList {
                vms: name.to_string(),
                locked: Some(locked.to_string()),
            });
        } else {
            let vm = line.trim().split('-').collect::<Vec<&str>>()[1]
                .split("(")
                .collect::<Vec<&str>>()[0];
            vms.push(VmList {
                vms: vm.to_string(),
                locked: None,
            });
        }
    }
    Ok(vms)
}

#[tauri::command]
fn turn_on_vm(name: String) -> Result<CodeMessage> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
                .arg("vd").arg("start").arg(name)
                .status()?
    } else {
        std::process::Command::new("sh")
                .arg("vd").arg("start").arg(name)
                .status()?
    };

    let msg = match output.code() {
        None => CodeMessage {
            message: "开机失败".to_string(),
            code: "1".to_string(),
        },
        Some(0) => CodeMessage {
            message: "开机成功".to_string(),
            code: "0".to_string(),
        },
        _ => CodeMessage {
            message: "开机失败,设备已开机".to_string(),
            code: "1".to_string(),
        },
    };
    Ok(msg)
}

// 关机
#[tauri::command]
fn turn_off_vm(name: String) ->Result<CodeMessage> {
    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
                .arg("vd").arg("stop").arg(name).arg("-y")
                .status()?
    } else {
        std::process::Command::new("sh")
                .arg("vd").arg("stop").arg(name).arg("-y")
                .status()?
    };

    let msg = match output.code() {
        None => CodeMessage {
            message: "关机失败".to_string(),
            code: "1".to_string(),
        },
        Some(0) => CodeMessage {
            message: "关机成功".to_string(),
            code: "0".to_string(),
        },
        _ => CodeMessage {
            message: "关机失败,设备已关机".to_string(),
            code: "1".to_string(),
        },
    };
    Ok(msg)
}

// 启动画面
#[tauri::command]
fn boot_screen(name: String) -> Result<()> {
    let _result = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
                .arg("vd").arg("spice").arg(name)
                .spawn()?
    } else {
        std::process::Command::new("sh")
                .arg("vd").arg("spice").arg(name)
                .spawn()?
    };
    println!("result: {:?}", _result);
    Ok(())
}


// 判断是否存在配置文件
#[tauri::command]
fn is_exist_config() -> Result<bool> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    if !config_dir.exists() {
        Ok(false)
    } else {
        let config_path = config_dir.join("vdesk.yaml");
        if config_path.exists() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            read_yaml_file,
            save_yaml_file,
            list_vm,
            turn_on_vm,
            boot_screen,
            turn_off_vm,
            is_exist_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
