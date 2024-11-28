// 创建 error 类型，表示程序中可能出现的所有错误
#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),
  #[error(transparent)]
  Tauri(#[from] tauri::Error),
  #[error(transparent)]
  SerdeYaml(#[from] serde_yaml::Error),
  #[error(transparent)]
  TauriPluginHttp(#[from] tauri_plugin_http::reqwest::Error),
  #[error("请求失败")]
  RequestFailed,
  #[error(transparent)]
  Json(#[from] json::Error),
  #[error("配置不存在")]
  ConfigNotFound,
  #[error("远程桌面未找到")]
  RemoteViewerNotFound,
  #[error("未知错误")]
  Unknown(String),
  #[error("registry error: {0}")]
  Registry(String),
}

// 我们必须手动实现 serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}
