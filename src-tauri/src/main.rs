#![cfg_attr(all(target_os = "windows"), windows_subsystem = "windows")]
// use dirs;
// use serde_yaml;

use std::string::FromUtf8Error;
use std::io::Read;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

mod vd;
use vd::Machine;

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
    #[error("reqwest error: {0}")]
    Reqwest(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        reqwest::Error,
    ),
    #[error("json error: {0}")]
    Json(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        json::Error,
    ),
    #[error("anyhow error")]
    Anyhow(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        anyhow::Error,
    ),
    #[error("vd error")]
    Vd(
        #[serde_as(as = "DisplayFromStr")]
        #[from]
        vd::Error,
    ),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Config {
    appid: String,
    appsecret: String,
    url: String,
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SuccessMessage {
    message: String,
}

#[tauri::command]
fn read_yaml_file(path: String) -> Result<Config> {
    let config_file = std::fs::File::open(path)?;
    let config: serde_yaml::Value = serde_yaml::from_reader(config_file).unwrap();
    let config = serde_yaml::from_value(config)?;
    Ok(config)
}

#[tauri::command]
fn return_yaml_file() -> Result<Config> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    let config_path = config_dir.join("vdesk.yaml");
    let config_file = std::fs::File::open(config_path)?;
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
fn list_vm() -> Result<Vec<Machine>> {
    let vd = vd::Vd::new()?;
    let vms = vd.list()?;
    Ok(vms)
}

// 开机
#[tauri::command]
fn turn_on_vm(name: String) -> Result<()> {
    let vd = vd::Vd::new()?;
    let name = name.replace(" ", "");
    vd.start(&name)?;
    Ok(())
}

// 关机
#[tauri::command]
fn turn_off_vm(name: String) -> Result<()> {
    let vd = vd::Vd::new()?;
    let name: String = name.replace(" ", "");
    vd.stop(&name)?;
    Ok(())
}

// 强制关机
#[tauri::command]
fn force_off_vm(name: String) -> Result<()> {
    let vd = vd::Vd::new()?;
    let name: String = name.replace(" ", "");
    vd.do_login(&name)?;
    vd.force_stop(&name)?;
    Ok(())
}

// 锁定
#[tauri::command]
fn locked_vm(name: String) -> Result<()> {
    let vd = vd::Vd::new()?;
    let name: String = name.replace(" ", "");
    vd.lock(&name)?;
    Ok(())
}

// 解锁
#[tauri::command]
fn unlocked_vm(name: String) -> Result<()> {
    let vd = vd::Vd::new()?;
    let name: String = name.replace(" ", "");
    vd.unlock(&name)?;
    Ok(())
}

// 启动画面
#[tauri::command]
async fn boot_screen(name: String) -> Result<()> {
    let _output = if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
            .arg("-WindowStyle Hidden -Command Start-Process")
            .arg("vd").arg("spice").arg(&name)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()?
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("bash")
            .arg("-c")
            .arg(format!("nohup vd spice {} > /dev/null &", &name))
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()?
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("nohup vd spice {} > /dev/null &", &name))
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()?
    };
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

// 打开vdesk.yaml将配置文件中的url改为北京服务器url
#[tauri::command]
fn switch_bj_server() -> Result<()> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    let config_path = config_dir.join("vdesk.yaml");
    let mut config_file = std::fs::File::open(&config_path)?;
    let mut content = String::new();
    config_file.read_to_string(&mut content)?;
    content = content.replace("https://vdesk-tj.knd.io", "https://vdesk.knd.io");
    std::fs::write(config_path, content)?;
    Ok(())
}

// 打开vdesk.yaml将配置文件中的url改为天津服务器url
#[tauri::command]
fn switch_tj_server() -> Result<()> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    let config_path = config_dir.join("vdesk.yaml");
    let mut config_file = std::fs::File::open(&config_path)?;
    let mut content = String::new();
    config_file.read_to_string(&mut content)?;
    content = content.replace("https://vdesk.knd.io", "https://vdesk-tj.knd.io");
    std::fs::write(config_path, content)?;
    Ok(())
}


#[tauri::command]
fn save_server_yaml_file(server: String) -> Result<SuccessMessage> {
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
    let config_path = config_dir.join("server.yaml");
    if config_path.exists() {
        // 删除文件
        std::fs::remove_file(&config_path)?;
    }
    let config = serde_yaml::to_string(&server)?;
    std::fs::write(config_path, config)?;
    Ok(SuccessMessage {
        message: "保存成功".to_string(),
    })
}

#[tauri::command]
fn read_server_yaml_file() -> Result<String> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        Error::Fs(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "home dir not found",
        ))
    })?;
    let config_dir = home_dir.join(".config");
    let config_path = config_dir.join("server.yaml");
    let config_file = std::fs::File::open(config_path)?;
    let config: serde_yaml::Value = serde_yaml::from_reader(config_file).unwrap();
    let config = serde_yaml::from_value(config)?;
    Ok(config)
}



fn main() {
    let _ = fix_path_env::fix();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_yaml_file,
            save_yaml_file,
            list_vm,
            turn_on_vm,
            boot_screen,
            turn_off_vm,
            is_exist_config,
            force_off_vm,
            locked_vm,
            unlocked_vm,
            switch_tj_server,
            switch_bj_server,
            return_yaml_file,
            save_server_yaml_file,
            read_server_yaml_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
