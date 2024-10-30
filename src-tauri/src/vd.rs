use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
use std::sync::Arc;

#[cfg(windows)]
use registry::{Data, Hive, Security};

#[cfg(windows)]
use anyhow::anyhow;
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    appid: String,
    appsecret: String,
    url: String,
    name: String,
}

#[derive(Clone, Debug)]
pub struct Vd {
    client: reqwest::Client,  // 改为异步客户端
    config: Config,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("config file format error: {0}")]
    Config(#[from] serde_yaml::Error),

    #[error("api call error: {0}")]
    Api(#[from] reqwest::Error),

    #[error("spice config error: {0}")]
    Json(#[from] json::Error),

    #[error("unknown error")]
    Unknown,
}

pub type Result<T> = std::result::Result<T, Error>;

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

impl Vd {
    pub async fn new() -> Result<Self> {
        let config = dirs::home_dir()
            .ok_or(Error::Unknown)?
            .join(".config")
            .join("vdesk.yaml");

        let config_content = fs::read_to_string(config).await?;
        let config: Config = serde_yaml::from_str(&config_content)?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        Ok(Self { config, client })
    }

    fn request<S: AsRef<str>>(
        &self,
        method: reqwest::Method,
        path: S,
    ) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.config.url, path.as_ref());
        self.client
            .request(method, url)
            .basic_auth(&self.config.appid, Some(&self.config.appsecret))
    }

    fn get<S: AsRef<str>>(&self, path: S) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::GET, path)
    }

    fn post<S: AsRef<str>>(&self, path: S) -> reqwest::RequestBuilder {
        self.request(reqwest::Method::POST, path)
    }

    pub async fn list(&self) -> Result<Vec<Machine>> {
        let machines: Vec<Machine> = self.get("/machines/").send().await?.json().await?;
        Ok(machines)
    }

    async fn vm_op<V: AsRef<str>, O: AsRef<str>>(
        &self,
        name: V,
        op: O,
    ) -> reqwest::Result<reqwest::Response> {
        self.post(format!("/machines/{}/{}", name.as_ref(), op.as_ref()))
            .send()
            .await
    }

    async fn vm_simple_op<V: AsRef<str>, O: AsRef<str>>(&self, name: V, op: O) -> Result<()> {
        self.vm_op(name, op).await?.error_for_status()?;
        Ok(())
    }

    pub async fn start<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "start").await
    }

    pub async fn stop<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "stop").await
    }

    pub async fn lock<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "lock").await
    }

    pub async fn unlock<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "unlock").await
    }

    pub async fn spice<V: AsRef<str>>(&self, name: V) -> Result<SpiceConfig> {
        let response = self.vm_op(name, "spice").await?;
        let text = response.text().await?;
        let configs: SpiceConfig = json::parse(&text)?;
        Ok(configs)
    }

    #[cfg(windows)]
    async fn remote_viewer_path() -> anyhow::Result<PathBuf> {
        // 由于 registry 操作是同步的，我们需要在一个阻塞任务中执行它
        tokio::task::spawn_blocking(|| {
            let key = Hive::LocalMachine.open(
                r"SOFTWARE\Classes\VirtViewer.vvfile\shell\open\command",
                Security::Read,
            )?;
            let data = key.value("")?;
            let s = match data {
                Data::String(s) => s.to_string_lossy(),
                _ => return Err(anyhow!("unexpected remote-viewer path")),
            };
            Ok(PathBuf::from(s[1..s.len() - 6].to_string()))
        }).await?
    }

    #[cfg(not(windows))]
    async fn remote_viewer_path() -> anyhow::Result<PathBuf> {
        Ok(PathBuf::from("remote-viewer"))
    }

    pub async fn do_login<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
        let name = name.as_ref();
        let _ = self.start(name).await;

        self.lock(name).await?;

        let vd = Arc::new(self.clone());
        let name_clone = name.to_string();

        // 创建清理函数
        let _cleanup = defer::defer(move || {
            let vd = vd.clone();
            let name = name_clone.clone();
            tokio::spawn(async move {
                let _ = vd.unlock(name).await;
            });
        });

        let mut config = self.spice(name).await?;
        config.insert("title", name)?;
        config.insert("auto-resize", "mever")?;
        config.insert("debug", false)?;
        config.insert("cursor", "MODE")?;
        config.insert("full-screen", true)?;

        let temp = std::env::temp_dir();
        let remote_viewer_config = temp.join(format!("__vd-remote-viewer-config-{}__", name));

        // 文件操作改为异步
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

        // 运行外部命令
        let remote_viewer = Self::remote_viewer_path().await?;
        tokio::process::Command::new(remote_viewer)
            .arg(remote_viewer_config)
            .spawn()?
            .wait()
            .await?;

        Ok(())
    }

    // pub async fn login<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
    //     let exe = std::env::current_exe()?;
    //
    //     tokio::process::Command::new(exe)
    //         .arg("--login")
    //         .arg(name.as_ref())
    //         .spawn()?;
    //
    //     Ok(())
    // }

    pub async fn force_stop<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
        let name = name.as_ref();
        self.stop(name).await?;
        self.unlock(name).await?;
        Ok(())
    }
}
