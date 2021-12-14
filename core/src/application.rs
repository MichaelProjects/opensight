use super::schema::applications;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct ApplicationData {
    pub application_name: String,
    pub os: String
}

#[derive(Serialize, Clone, Debug, Hash, Queryable, Insertable)]
pub struct Application {
    pub application_id: String,
    pub application_name: String,
    pub creation_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}
impl Application {
    pub fn new(name: &str, os: ApplicationType) -> Application {
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

fn create_token(app: &Application) -> String {
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}
