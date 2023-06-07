use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use thiserror::Error;

#[cfg(windows)]
use registry::{Data, Hive, Security};

#[cfg(windows)]
use anyhow::anyhow;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    appid: String,
    appsecret: String,
    url: String,
    name: String,
}

#[derive(Clone, Debug)]
pub struct Vd {
    client: reqwest::blocking::Client,
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
    pub fn new() -> Result<Self> {
        let config = dirs::home_dir()
            .ok_or(Error::Unknown)?
            .join(".config")
            .join("vdesk.yaml");
        let config: Config = serde_yaml::from_str(std::fs::read_to_string(config)?.as_str())?;
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;
        Ok(Self { config, client })
    }

    fn request<S: AsRef<str>>(
        &self,
        method: reqwest::Method,
        path: S,
    ) -> reqwest::blocking::RequestBuilder {
        let url = format!("{}{}", self.config.url, path.as_ref());
        self.client
            .request(method, url)
            .basic_auth(&self.config.appid, Some(&self.config.appsecret))
    }

    fn get<S: AsRef<str>>(&self, path: S) -> reqwest::blocking::RequestBuilder {
        self.request(reqwest::Method::GET, path)
    }

    fn post<S: AsRef<str>>(&self, path: S) -> reqwest::blocking::RequestBuilder {
        self.request(reqwest::Method::POST, path)
    }

    pub fn list(&self) -> Result<Vec<Machine>> {
        let machines: Vec<Machine> = self.get("/machines/").send()?.json()?;
        Ok(machines)
    }

    fn vm_op<V: AsRef<str>, O: AsRef<str>>(
        &self,
        name: V,
        op: O,
    ) -> reqwest::Result<reqwest::blocking::Response> {
        self.post(format!("/machines/{}/{}", name.as_ref(), op.as_ref()))
            .send()
    }

    fn vm_simple_op<V: AsRef<str>, O: AsRef<str>>(&self, name: V, op: O) -> Result<()> {
        self.vm_op(name, op)?.error_for_status()?;
        Ok(())
    }

    pub fn start<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "start")
    }

    pub fn stop<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "stop")
    }

    pub fn lock<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "lock")
    }

    pub fn unlock<V: AsRef<str>>(&self, name: V) -> Result<()> {
        self.vm_simple_op(name, "unlock")
    }

    pub fn spice<V: AsRef<str>>(&self, name: V) -> Result<SpiceConfig> {
        let configs: SpiceConfig = json::parse(self.vm_op(name, "spice")?.text()?.as_str())?;
        Ok(configs)
    }

    #[cfg(windows)]
    fn remote_viewer_path() -> anyhow::Result<PathBuf> {
        let key = Hive::LocalMachine.open(
            r"SOFTWARE\Classes\VirtViewer.vvfile\shell\open\command",
            Security::Read,
        )?;
        let data = key.value("")?;
        // "C:\Program Files\VirtViewer v11.0-256\bin\remote-viewer.exe" "%1"
        let s = match data {
            Data::String(s) => s.to_string_lossy(),
            _ => return Err(anyhow!("unexpected remote-viewer path")),
        };

        Ok(PathBuf::from(s[1..s.len() - 6].to_string()))
    }

    #[cfg(not(windows))]
    fn remote_viewer_path() -> anyhow::Result<PathBuf> {
        Ok(PathBuf::from("remote-viewer"))
    }

    pub fn do_login<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
        let name = name.as_ref();
        let _ = self.start(name);

        self.lock(name)?;

        let _cleanup = defer::defer(|| {
            let _ = self.unlock(name);
        });

        let mut config = self.spice(name)?;
        config.insert("title", name)?;

        let temp = std::env::temp_dir();
        let remote_viewer_config = temp.join(format!("__vd-remote-viewer-config-{}__", name));

        {
            let f = std::fs::File::create(remote_viewer_config.as_path())?;
            let mut f = LineWriter::new(f);
            f.write_all(b"[virt-viewer]\n")?;
            if let SpiceConfig::Object(obj) = config {
                for (k, v) in obj.iter() {
                    f.write_all(format!("{}={}\n", k, v).as_bytes())?;
                }
            };

            f.flush()?;
        }

        subprocess::Exec::cmd(Self::remote_viewer_path()?)
            .arg(remote_viewer_config)
            .popen()?
            .wait()?;
        Ok(())
    }

    pub fn login<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
        let exe = std::env::current_exe()?;

        subprocess::Exec::cmd(exe)
            .arg("--login")
            .arg(name.as_ref())
            .detached()
            .popen()?;

        Ok(())
    }

    pub fn force_stop<V: AsRef<str>>(&self, name: V) -> anyhow::Result<()> {
        let name = name.as_ref();
        self.stop(name)?;
        self.unlock(name)?;

        Ok(())
    }
}
