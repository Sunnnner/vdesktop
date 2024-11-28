use std::{path::PathBuf, sync::Arc};
use json::JsonValue;
use serde::{Serialize, Deserialize};

use super::config::ConfigManager;




#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SuccessMessage {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub config_dir: Arc<PathBuf>,
    pub is_exist_config: bool,
    pub config_manager: ConfigManager,
    pub temp_dir: Arc<PathBuf>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub ip: std::net::Ipv4Addr,
    pub name: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Machine {
    pub id: i32,
    pub name: String,
    pub vmid: String,
    pub node: Node,
    pub locked_by: Option<User>,
}

pub type SpiceConfig = JsonValue;