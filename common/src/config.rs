use once_cell::sync::OnceCell;
use serde::Serialize;

pub const VERSION: &str = "0.0.1";

#[derive(Debug, Default, Clone)]
pub struct AppConfig {
    pub port: i32,
    pub db_path: String,
    pub log_path: String,
    pub mode: ServerMode,
    pub version: String,
}

// pub static  <AppConfig>
pub static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum ServerMode {
    Debug,
    Test,
    Release,
}

impl Default for ServerMode {
    fn default() -> Self {
        ServerMode::Release
    }
}

impl From<&str> for ServerMode {
    fn from(value: &str) -> Self {
        match value {
            "debug" => ServerMode::Debug,
            "test" => ServerMode::Test,
            _ => ServerMode::Release,
        }
    }
}
