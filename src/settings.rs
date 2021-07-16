use std::env;
use serde::{Deserialize};
use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Database {
    pub postgresql_url: String,
    pub postgresql_user: String,
    pub postgresql_port: String,
    pub postgresql_dbname: String,
    pub postgresql_password: String
}

#[derive(Debug, Deserialize)]
pub struct Settings{
    pub database: Database,
    pub debug: bool
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError>{
        let mut s = Config::default();
        s.merge(File::with_name("conf/default"))?;

        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("conf/{}", env)).required(false))?;

        s.try_into()
    }
}