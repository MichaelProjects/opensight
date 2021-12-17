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

#[derive(Serialize, Clone, Debug, Hash)]
pub struct Application {    
    pub application_id: String,
    pub application_name: String,
    pub creation_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}

impl Application {
    pub fn from_str(s: String) -> Result<Application, Box<dyn std::error::Error>> {
        let a: Value = serde_json::from_str(&s)?;
        let app = Application{
            application_id: a["application_id"].as_str().unwrap().to_string(),
            application_name: a["application_name"].as_str().unwrap().to_string(),
            creation_time: NaiveDateTime::from_timestamp(a["creation_time"].as_i64().unwrap(), 0),
            token: a["token"].as_str().unwrap().to_string(),
            os: a["os"].as_str().unwrap().to_string(),
        };
        Ok(app)
    }
    pub async fn get(conf: &Settings, application_id: String) -> Result<Application, Box<dyn std::error::Error>> {
        let app_data = application_dao::get_application(&conf, &application_id).await?;
        let app = Application::from_str(app_data)?;
        Ok(app)
    }
    pub async fn get_all(conf: &Settings) -> Result<Application, Box<dyn std::error::Error>> {
        let app_data = application_dao::get_application(&conf).await?;
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
