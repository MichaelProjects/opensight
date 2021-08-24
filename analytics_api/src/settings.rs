
use std::env;
use serde::{Deserialize};
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub connection_string: String
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub debug: bool,
    pub port: u16,
}
#[derive(Debug, Deserialize)]
pub struct Settings{
    pub database: Database,
    pub general: General,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError>{
        let mut s = Config::default();
        s.merge(File::with_name("config"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("conf/{}", env)).required(false))?;

        s.try_into()
    }
}