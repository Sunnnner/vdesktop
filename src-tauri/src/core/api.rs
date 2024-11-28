use tauri::Manager;

use super::{config::Config, model::{AppState, Machine, SuccessMessage}, vd::Vd};
use crate::Result;

#[tauri::command]
pub async fn get_config(app_handle: tauri::AppHandle) -> Result<Config> {
    let config_manager = app_handle.state::<AppState>().config_manager.clone();
    config_manager.get().await
}

#[tauri::command]
pub async fn save_config(app_handle: tauri::AppHandle, config: Config) -> Result<SuccessMessage> {
  let config_manager = app_handle.state::<AppState>().config_manager.clone();
  config_manager.save(config).await?;
  Ok(SuccessMessage{
    message: "保存成功".to_string()
  })  
}

#[tauri::command]
pub async fn is_exist_config(app_handle: tauri::AppHandle) -> Result<bool> {
  let exists = app_handle.state::<AppState>().is_exist_config;
  Ok(exists)
}

#[tauri::command]
pub async fn switch_server(app_handle: tauri::AppHandle, server: String) -> Result<SuccessMessage> {
  let config_manager = app_handle.state::<AppState>().config_manager.clone();
  config_manager.switch_server(server).await?;
  Ok(SuccessMessage{
    message: "切换成功".to_string()
  })
}

#[tauri::command]
pub async fn get_vms(app_handle: tauri::AppHandle) -> Result<Vec<Machine>> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.list().await
}

#[tauri::command]
pub async fn spice_viewer(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.start(&name).await?;
    vd.lock(&name).await?;
    vd.spice_viewer(&name).await?;
    Ok(SuccessMessage{
      message: "启动成功".to_string()
    })
}

#[tauri::command]
pub async fn start_vms(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.start(&name).await?;
    Ok(SuccessMessage{
      message: "启动成功".to_string()
  })
}

#[tauri::command]
pub async fn stop_vms(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.stop(&name).await?;

    Ok(SuccessMessage{
      message: "关机成功".to_string()
    })
}

#[tauri::command]
pub async fn lock_vms(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.lock(&name).await?;
    Ok(SuccessMessage{
      message: "锁定成功".to_string()
    })
}

#[tauri::command]
pub async fn unlock_vms(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.unlock(&name).await?;
    Ok(SuccessMessage{
      message: "解锁成功".to_string()
    })
}

#[tauri::command]
pub async fn force_stop_vms(app_handle: tauri::AppHandle, name: String) -> Result<SuccessMessage> {
    let path = app_handle.state::<AppState>().config_dir.clone();
    let config = Config::get(&path).await?;
    let vd = Vd::new(config)?;
    vd.force_stop(&name).await?;
    Ok(SuccessMessage{
      message: "强制关机成功".to_string()
    })
}   