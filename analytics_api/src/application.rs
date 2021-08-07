use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use postgres::{Client, NoTls};
use crate::application_dao::ApplicationDao;
use crate::dao::Dao;


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
#[derive(Hash, Debug)]
pub struct Application {
    pub name: String,
    pub os: ApplicationType,
    pub uuid: String,
    pub added: DateTime<Utc>,
    pub token: String,
}


impl Application{
    pub fn new(name: &str, os: ApplicationType) -> Application{
        let uuid: String = Uuid::new_v4().to_string();
        let get_time = Utc::now();
        let mut app = Application{name: String::from(name), os, uuid, added: get_time, token: String::new()};
        app.token = create_token(&app);
        app
    }
}

pub fn insert_entry(app: Application, conn: &mut Client) -> bool{
    let mut successful = true;
    let query = "INSERT INTO applications (application_id, application_name, created_time, token, os) values ($1,$2,$3,$4,$5,$6)".to_string();
    let response = match conn.execute(query.as_str(),&[&app.uuid, &app.name, &app.added, &app.token, &app.os.as_str()]){
        Ok(response) => response,
        Err(err) => panic!("Error while inserting: {}", err)
    };
    println!("Rows Affected: {}", response);
    if response == 0{
        successful = false;
    }
    successful
}


pub fn get_application_details(connection_str: String)  {
    let mut client =  match Client::connect(connection_str.as_str(), NoTls) {
        Ok(client) => client,
        Err(e) => panic!("Error while connecting to db: {}", e)
    };
    let application_dao = ApplicationDao::new();
    let applications: Vec<Application> = application_dao.get_entry(&mut client);
    for app in applications.iter() {
        println!("Application {:?}", app);
    }
}


fn create_token(app: &Application) -> String{
    let mut s = DefaultHasher::new();
    app.hash(&mut s);
    s.finish().to_string()
}