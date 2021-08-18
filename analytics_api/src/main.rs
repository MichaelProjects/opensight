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

use crate::settings::{Settings};
use crate::db::*;
use handler::{get_health, insert_entry};
use crate::application::{Application, get_application_details, get_all_apps};

pub struct Storage{
    pub apps: Vec<Application>,
}

pub async fn create_routes(conf: Settings, app: Storage){
    rocket::build()
        .attach(AnalyticsDB::fairing())
        .manage( conf)
        .mount("/analytic", routes![get_health, insert_entry] )
        .launch()
        .await;
}
#[rocket::main]
async fn main(){
    env_logger::init();
    let conf = Settings::new().unwrap();
    let apps = get_all_apps("postgres://analyze_account:Glc95FLYbkgQwCy5KwUu@localhost/postgres");
    let storage = Storage{apps};
    create_routes(conf, storage).await;
}