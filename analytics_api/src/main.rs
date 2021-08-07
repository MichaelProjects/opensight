#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod health;
mod settings;
mod analytics;
mod migration;
mod application;
mod application_dao;
mod dao;
use rocket_sync_db_pools::{database, postgres};
use crate::settings::{Settings};
use crate::analytics::{AnalyticData, AnalyticEntry};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::State;
use std::thread;
use log::{info};
use crate::migration::check_db_tables;
use crate::application::{get_application_details};
use serde_json::Value;
use crate::application_dao::ApplicationDao;
use crate::dao::Dao;

struct DBPost{
    url: String
}

#[database("postgres_url")]
struct AnalyticsDB(postgres::Client);

#[derive(Serialize, Deserialize)]
struct Response{
    message: String,
    error: bool
}
impl Response{
    pub fn new(message: &str, error: bool) -> Self{
        Response{message: String::from(message), error}
    }
}

#[get("/health")]
async fn get_health(conn: AnalyticsDB, con_str: &State<DBPost>) -> Json<health::Health>{
    Json(health::get_health_state(con_str.url.clone()))
}

#[post("/analytics/<application_id>/entry", data="<analytics>")]
async fn insert_entry(conn: AnalyticsDB, application_id: String, analytics: Json<AnalyticData>) -> Json<Response>{
    let analytic_entry = AnalyticEntry::new(application_id, analytics.creation_date, analytics.os.clone(), analytics.device_size.clone(), analytics.session_id.clone(), analytics.session_length);
    let result = conn.run(|c| analytic_entry.insert_entry(c)).await;
    info!("{}", result);
    Json(Response::new("a",  false))
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let conf = Settings::new().unwrap();

    let connection_str = migration::build_connection_st(conf.database.postgresql_url.clone(),
                                                        String::from("postgres"),
                                                        conf.database.postgresql_user.clone(),
                                                        conf.database.postgresql_password.clone());
    let a = connection_str.clone();
    let b = connection_str.clone();

    thread::spawn(||{
        get_application_details(a);
    });
    thread::spawn(||{
        check_db_tables(b);
    });
    rocket::build()
        .manage(DBPost{ url: connection_str.clone()})
        .attach(AnalyticsDB::fairing())
        .manage( conf)
        .mount("/", routes![get_health, insert_entry] )
}