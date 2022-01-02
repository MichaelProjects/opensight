use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub connection_string: String,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub debug: bool,
    pub port: u16,
    pub address: String,
    pub log_file: String,
    pub opensight_core: String,
}
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub general: General,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name("conf"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("conf/{}", env)).required(false))?;

        s.try_into()
    }
}
