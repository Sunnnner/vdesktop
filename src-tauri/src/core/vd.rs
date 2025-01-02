use tauri::http;
use tokio::io::AsyncWriteExt;

use super::{config::Config, model::{Machine, SpiceConfig}};
use crate::{utils::error::Error, Result};
use std::{path::PathBuf, sync::Arc};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
#[cfg(windows)]
use registry::{Data, Hive, Security};

#[derive(Debug, Clone)]
pub struct Vd {
    pub client: tauri_plugin_http::reqwest::Client,
    pub config: Config,

}

impl Vd {
    pub fn new(config: Config) -> Result<Self> {
        let client = tauri_plugin_http::reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self { client, config })
    }

    pub async fn request(&self, method: http::Method, path: &str) -> tauri_plugin_http::reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.url, path);
        self.client.request(method, url).basic_auth(&self.config.appid, Some(&self.config.appsecret))
    }

    pub async fn get(&self, path: &str) -> tauri_plugin_http::reqwest::RequestBuilder {
        self.request(http::Method::GET, path).await
    }

    pub async fn post(&self, path: &str) -> tauri_plugin_http::reqwest::RequestBuilder {
        self.request(http::Method::POST, path).await
    }

    pub async fn list(&self) -> Result<Vec<Machine>> {
        let response = self.get("/machines/").await.send().await?;
        if !response.status().is_success() {
            return Err(Error::RequestFailed);
        }
        let res = response.json().await?;
        Ok(res)
    }

    pub async fn vm_op(&self, name: &str, op: &str) -> Result<tauri_plugin_http::reqwest::Response> {
        Ok(self.post(&format!("/machines/{}/{}", name, op)).await.send().await?)
    }

    pub async fn vm_simple_op(&self, name: &str, op: &str) -> Result<()> {
        self.vm_op(name, op).await?;
        Ok(())
    }

    pub async fn start(&self, name: &str) -> Result<()> {
        self.vm_simple_op(name, "start").await
    }

    pub async fn stop(&self, name: &str) -> Result<()> {
        self.vm_simple_op(name, "stop").await
    }

    pub async fn lock(&self, name: &str) -> Result<()> {
        self.vm_simple_op(name, "lock").await
    }

    pub async fn unlock(&self, name: &str) -> Result<()> {
        self.vm_simple_op(name, "unlock").await
    }

    pub async fn spice(&self, name: &str) -> Result<SpiceConfig> {
        let response = self.vm_op(name, "spice").await?;
        let text = response.text().await?;
        let configs: SpiceConfig = json::parse(&text)?;
        Ok(configs)
    }

    #[cfg(not(target_os = "windows"))]
    pub async fn remote_viewer_path() -> Result<PathBuf> {
        // 优先使用which查找remote-viewer
        if let Ok(path) = which::which("remote-viewer") {
            return Ok(path);
        }
        
        // 检查常见安装路径
        let possible_paths = vec![
            "/usr/local/bin/remote-viewer",
            "/opt/local/bin/remote-viewer",
            "/usr/bin/remote-viewer"
        ];

        for path in possible_paths {
            if Path::new(path).exists() {
                return Ok(PathBuf::from(path));
            }
        }

        Err(Error::RemoteViewerNotFound)
    }

    #[cfg(windows)]
    async fn remote_viewer_path() -> Result<PathBuf> {
        tokio::task::spawn_blocking(|| {
            let key = Hive::LocalMachine.open(
                r"SOFTWARE\Classes\VirtViewer.vvfile\shell\open\command",
                Security::Read,
            ).map_err(|e| Error::Registry(format!("hive open error: {}", e)))?;
            
            let data = key.value("").map_err(|e| Error::Registry(format!("key value error: {}", e)))?;
            
            match data {
                Data::String(s) => {
                    let path_str = s.to_string_lossy();
                    if path_str.len() < 7 {
                        return Err(Error::RemoteViewerNotFound);
                    }
                    Ok(PathBuf::from(path_str[1..path_str.len() - 6].to_string()))
                }
                _ => Err(Error::RemoteViewerNotFound),
            }
        }).await.map_err(|e| Error::Unknown(e.to_string()))?
    }


    pub async fn spice_viewer(&self, name: &str, temp: &Path) -> Result<()> {
        let vd = Arc::new(self.clone());
        let name_clone = name.to_string();
    
        let _cleanup = defer::defer(move || {
            let vd = vd.clone();
            tokio::spawn(async move {
                let _ = vd.unlock(&name_clone).await;
            });
        });
    
        let mut config = self.spice(name).await?;
        
        // 配置设置
        config.insert("hotkeys", "")?;

        let remote_viewer_config = temp.join(format!("__vd-remote-viewer-config-{}__.vv", name));
        // 写入配置
        {
            let mut file = tokio::fs::File::create(&remote_viewer_config).await?;
            file.write_all(b"[virt-viewer]\n").await?;
            if let SpiceConfig::Object(obj) = config {
                for (k, v) in obj.iter() {
                    file.write_all(format!("{}={}\n", k, v).as_bytes()).await?;
                }
            };
            file.flush().await?;
        }
    
        // 设置配置文件权限
        tokio::fs::set_permissions(&remote_viewer_config, std::fs::Permissions::from_mode(0o644)).await?;
    
        let remote_viewer = Self::remote_viewer_path().await?;
    
        // 直接执行remote-viewer
        let mut command = tokio::process::Command::new(&remote_viewer);
        command
            .arg(&remote_viewer_config)
            .arg("--class=vd-remote-viewer")           // 设置唯一的程序类名
            .arg(&format!("--name=vd-viewer-{}", name)) // 使用动态名称区分不同实例
            .arg("--gtk-module=gail:atk-bridge")  
            .env("GTK_CSD", "0") 
            .env("GTK_THEME", "Adwaita:light")
            .env("GTK2_RC_FILES", "")
            .env("GTK_OVERLAY_SCROLLING", "1")
            .env("GTK_MODULES", "")
            .arg("--auto-resize=never")              // 命令行强制禁用自动缩放
            .arg("--spice-disable-audio")
            .arg("--cursor=local")                   // 本地光标
            .arg("--spice-disable-effects=all")
            .arg("--spice-disable-usbredir")
            .arg("--spice-preferred-compression=lz4")  // 关闭压缩，直接传输
            .arg("--spice-disable-effects=all")        // 禁用所有视觉效果
            .env("PATH", "/usr/local/bin:/opt/local/bin:/usr/bin:/bin")
            .spawn()?;
        Ok(())
    }
    

    pub async fn force_stop(&self, name: &str) -> Result<()> {
        self.stop(name).await?;
        self.unlock(name).await?;
        Ok(())
    }

}
