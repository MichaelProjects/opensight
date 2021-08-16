#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod health;
mod settings;
mod analytics;
mod application;
mod application_dao;
mod dao;
mod schema;

use rocket_sync_db_pools::{database};
use crate::settings::{Settings};
use crate::analytics::{AnalyticData, AnalyticEntry};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::State;
use log::{info};


#[database("postgres_url")]
struct AnalyticsDB(diesel::PgConnection);

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
async fn get_health(conn: AnalyticsDB) -> Json<health::Health>{
    Json(health::get_health_state())
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
        // how to manage state in rust         .manage(DBPost{ url: connection_str.clone()})
    rocket::build()
        .attach(AnalyticsDB::fairing())
        .manage( conf)
        .mount("/", routes![get_health, insert_entry] )
}