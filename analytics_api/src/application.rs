use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::application_dao;
use crate::settings::Settings;

#[derive(Serialize, Deserialize)]
pub struct ApplicationData<'a> {
    pub application_name: &'a str,
    pub os: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Application {    
    pub application_id: String,
    pub application_name: String,
    pub creation_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}

impl Application {

    pub async fn get(conf: &Settings, application_id: String) -> Result<Application, Box<dyn std::error::Error>> {
        let app = application_dao::get(&conf, &application_id).await?;
        Ok(app)
    }
    pub async fn get_all(conf: &Settings) -> Result<Vec<Application>, Box<dyn std::error::Error>> {
        let response = application_dao::get_all(&conf).await?;
        Ok(response)
    }
}

#[derive(Hash, Debug)]
pub enum ApplicationType {
    IOS,
    Android,
    Web,
    NotFound,
}
impl ApplicationType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ApplicationType::IOS => "ios",
            ApplicationType::Android => "android",
            ApplicationType::Web => "web",
            ApplicationType::NotFound => "notFound",
        }
    }
    pub fn from_str(str: &str) -> ApplicationType {
        match str {
            "ios" => ApplicationType::IOS,
            "android" => ApplicationType::Android,
            "web" => ApplicationType::Web,
            "notFound" => ApplicationType::NotFound,
            _ => ApplicationType::NotFound,
        }
    }
}

fn create_token(app: Application) -> String {
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}
