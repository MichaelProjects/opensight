use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use crate::daos::application_dao;
use crate::settings::Settings;

#[derive(Serialize, Deserialize, Clone, Debug)]
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
        let response = application_dao::get_all(&conf.general.opensight_core).await?;
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
