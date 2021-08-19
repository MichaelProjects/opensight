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
mod handler;
mod db;
mod analytics_dao;
mod cache;

use crate::settings::{Settings};
use crate::db::*;
use handler::{get_health, insert_entry, insert_application};
use crate::application::{Application, get_all_apps};
use std::sync::Mutex;
use crate::cache::Cache;


pub async fn create_routes(conf: Settings, app: Cache){
    rocket::build()
        .attach(AnalyticsDB::fairing())
        .manage( conf)
        .manage(app)
        .mount("/analytic", routes![get_health, insert_entry, insert_application] )
        .launch()
        .await;
}
#[rocket::main]
async fn main(){
    env_logger::init();
    let conf = Settings::new().unwrap();
    let apps = get_all_apps("postgres://analyze_account:Glc95FLYbkgQwCy5KwUu@localhost/postgres");
    create_routes(conf, cache::Cache{all_apps: apps}).await;
}