use std::{path::MAIN_SEPARATOR, sync::OnceLock};

use serde::Serialize;

pub const VERSION: &str = "0.0.1";

#[derive(Debug, Default, Clone)]
pub struct AppConfig {
    pub port: i32,
    pub workspace: String,
    pub log: String,
    pub database: String,
    pub mode: ServerMode,
    pub version: String,
}

impl AppConfig {
    pub fn init(&mut self) {
        let mut db = self.workspace.clone();
        db.push(MAIN_SEPARATOR);
        db.push_str("database");
        db.push(MAIN_SEPARATOR);
        self.database = db;

        let mut log = self.workspace.clone();
        log.push(MAIN_SEPARATOR);
        log.push_str("logs");
        log.push(MAIN_SEPARATOR);
        self.log = log;
    }

    pub fn db_local_path(&self) -> String {
        let mut path = String::from("sqlite://");
        path.push_str(self.database.clone().as_str());
        path.push_str("local");
        path
    }
}

// pub static  <AppConfig>
pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Default)]
#[serde(untagged)]
pub enum ServerMode {
    Debug,
    Test,
    #[default]
    Release,
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
