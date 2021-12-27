extern crate diesel;

use super::schema::applications;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;
use crate::application_dao::{insert_application, get_application, get_all_applications};
use crate::db::DatabaseConnection;
use std::error::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub name: String,
    pub package_name: String,
    pub os: ApplicationType
}

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
#[table_name = "applications"]
pub struct Application {    
    pub application_id: String,
    pub application_name: String,
    pub package_name: String,
    pub creation_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}
impl Application {
    pub fn new(name: &String, os: &ApplicationType, package_name: &String) -> Application {
        let application_id: String = Uuid::new_v4().to_string();
        let get_time = Utc::now().naive_utc();
        let mut app = Application {
            application_name: String::from(name),
            package_name: package_name.clone(),
            os: String::from(os.as_str()),
            application_id,
            creation_time: get_time,
            token: String::new(),
        };
        app.token = create_token(&app);
        app
    }
    pub async fn insert(app: Application, conn: DatabaseConnection) -> Result<Application, Box<dyn Error>> {
        let response = conn.run(|c| insert_application(app, c)).await?;
        Ok(response)
    }

    pub async fn get(app_id: String, conn: DatabaseConnection) -> Result<Application, Box<dyn Error>>{
        let app = conn.run(|c| get_application(app_id, c)).await?;
        Ok(app)
    }
    
    pub async fn get_all(conn: &DatabaseConnection) -> Result<Vec<Application>, Box<dyn Error>> {
        let apps = conn.run(|c| get_all_applications(c)).await?;
        Ok(apps)
    }
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
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
        match str.to_lowercase().as_str() {
            "ios" => ApplicationType::IOS,
            "android" => ApplicationType::Android,
            "web" => ApplicationType::Web,
            "notFound" => ApplicationType::NotFound,
            _ => ApplicationType::NotFound,
        }
    }
}

fn create_token(app: &Application) -> String {
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}
