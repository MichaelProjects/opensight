use diesel;

use super::schema::applications;

use chrono::{Utc, NaiveDateTime};
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use crate::application_dao::ApplicationDao;
use crate::dao::Dao;
use diesel::{PgConnection, Connection};

#[derive(Clone, Debug, Hash, Queryable, AsChangeset)]
#[table_name="applications"]
pub struct Application {
    pub application_id: String,
    pub application_name: String,
    pub created_time: NaiveDateTime,
    pub token: String,
    pub os: String,
}

impl Application{
    pub fn new(name: &str, os: ApplicationType) -> Application{
        let application_id: String = Uuid::new_v4().to_string();
        let get_time = Utc::now().naive_utc();
        let mut app = Application{application_name: String::from(name), os: String::from(os.as_str()), application_id, created_time: get_time, token: String::new()};
        app.token = create_token(app.clone());
        app
    }
    pub fn insert_entry(app: Application, conn: &mut PgConnection) -> bool{
        let app_dao = ApplicationDao::new();
        app_dao.insert_entry(app, conn)
    }
}

#[derive(Hash, Debug)]
pub enum ApplicationType { IOS, Android, Web, NotFound }
impl ApplicationType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ApplicationType::IOS => "ios",
            ApplicationType::Android => "android",
            ApplicationType::Web => "web",
            ApplicationType::NotFound => "notFound",
        }
    }
    pub fn from_str(str: &str) ->  ApplicationType {
        match str{
            "ios" => ApplicationType::IOS,
            "android" => ApplicationType::Android,
            "web" => ApplicationType::Web,
            "notFound" => ApplicationType::NotFound,
            _ => ApplicationType::NotFound
        }
    }
}

#[derive(Insertable)]
#[table_name="applications"]
pub struct InsertableApplication {
    application_name: String,
    os: String,
    created_time: NaiveDateTime,
    token: String
}

impl InsertableApplication {
    pub fn from_application(app: Application) -> InsertableApplication {
        InsertableApplication{
            application_name: app.application_name,
            os: app.os,
            created_time: app.created_time,
            token: app.token
        }
    }
}


pub fn get_application_details(connection_str: &str)  {
    let mut conn = diesel::PgConnection::establish(connection_str).unwrap();
    let application_dao = ApplicationDao::new();
    let applications: Vec<Application> = application_dao.get_entry(&mut conn);
    for app in applications.iter() {
        println!("Application {:?}", app);
    }
}

fn create_token(app: Application) -> String{
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}

