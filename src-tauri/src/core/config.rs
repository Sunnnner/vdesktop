use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use crate::Result;
use tokio::{fs, io::AsyncReadExt};
use tokio::sync::Mutex;
use std::sync::Arc;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub appid: String,
    pub appsecret: String,
    pub url: String,
    pub name: String,
    pub server: String,
}

#[derive(Debug, Clone)]
pub struct ConfigManager {
  config_path: PathBuf,
  file_lock: Arc<Mutex<()>>,
}


impl ConfigManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self { config_path, file_lock: Arc::new(Mutex::new(())) }
    }

    pub async fn get(&self) -> Result<Config> {
        let _lock = self.file_lock.lock().await;
        Config::get(&self.config_path).await
    }

    pub async fn save(&self, config: Config) -> Result<()> {
        let _lock = self.file_lock.lock().await;
        Config::save(&self.config_path, config).await
    }

    pub async fn switch_server(&self, server: String) -> Result<()> {
        let _lock = self.file_lock.lock().await;
        Config::switch_server(&self.config_path, server).await
    }
}


impl Config {

    pub async fn get(path: &PathBuf) -> Result<Self> {
      let mut config_file = fs::File::open(path).await?;
      let mut content = String::new();
      config_file.read_to_string(&mut content).await?;
      let config: Config = serde_yaml::from_str(&content)?;
      Ok(config)
    }


    pub async fn save(path: &PathBuf, config: Config) -> Result<()> {
        let config_string = serde_yaml::to_string(&config)?;
        fs::write(path, config_string).await?;
        Ok(())
    }

    pub async fn switch_server(path: &PathBuf, server: String) -> Result<()> {
        let mut config_file = fs::File::open(path).await?;
        let mut content = String::new();
        config_file.read_to_string(&mut content).await?;
        let mut config: Config = serde_yaml::from_str(&content)?;
        match server.as_str() {
          "beijing" => {
            config.url = "https://vdesk.knd.io".to_string();
          }
          "tianjing" => {
            config.url = "https://vdesk-tj.knd.io".to_string();
          }
            _ => {}
        }
        config.server = server;
        fs::write(path, serde_yaml::to_string(&config)?).await?;
        Ok(())
    }
}