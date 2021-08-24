#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

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
use diesel::prelude::*;
use crate::cache::Cache;
use rocket::{config, Config};
use std::net::{IpAddr, Ipv4Addr};
use rocket::config::LogLevel;
use rocket::figment::Figment;

pub fn insert_conf_values(conf: &Settings) -> Figment {
    Config::figment()
        .merge(("port", &conf.general.port))
        .merge(("databases.postgres_url.url", &conf.database.connection_string))
}

pub async fn create_routes(conf: Settings, app: Cache,){
    rocket::custom(insert_conf_values(&conf))
        .attach(AnalyticsDB::fairing())
        .manage(conf)
        .manage(app)
        .mount("/analytic", routes![get_health, insert_entry, insert_application] )
        .launch()
        .await;
}


embed_migrations!("./migrations/");

#[rocket::main]
async fn main(){
    env_logger::init();
    let conf = Settings::new().unwrap();
    // This will run the necessary migrations.
    let connection = PgConnection::establish(conf.database.connection_string.as_str()).unwrap();
    embedded_migrations::run(&connection);


    create_routes(conf, cache::Cache{all_apps: vec![]}).await;
}