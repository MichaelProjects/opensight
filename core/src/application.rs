extern crate diesel;

use super::schema::applications;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;
use crate::application_dao::{insert_application, get_application};
use crate::db::DatabaseConnection;


#[derive(Serialize, Deserialize, Clone)]
pub struct ApplicationData {
    pub name: String,
    pub os: ApplicationType
}

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
#[table_name = "applications"]
pub struct Application {    
    pub application_id: String,
    pub application_name: String,
    pub creation_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}
impl Application {
    pub fn new(name: &String, os: &ApplicationType) -> Application {
        let application_id: String = Uuid::new_v4().to_string();
        let get_time = Utc::now().naive_utc();
        let mut app = Application {
            application_name: String::from(name),
            os: String::from(os.as_str()),
            application_id,
            creation_time: get_time,
            token: String::new(),
        };
        app.token = create_token(&app);
        app
    }
    pub fn insert(self, conn: &PgConnection)  {
        insert_application(&self, conn);
    }
    pub async fn get(app_id: String, conn: DatabaseConnection) -> Application {
        let app = conn.run(|c| get_application(&app_id, c)).await;
        app
    }
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone)]
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

fn create_token(app: &Application) -> String {
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}
