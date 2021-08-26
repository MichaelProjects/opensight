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

use diesel::prelude::*;
use crate::cache::Cache;


use rocket::figment::Figment;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;

pub fn insert_conf_values(conf: &Settings) -> Figment {
    let mut logging_string = "critical";
    if &conf.general.debug == &true{
        logging_string = "debug";
    }
    rocket::Config::figment()
        .merge(("port", &conf.general.port))
        .merge(("address", &conf.general.address))
        .merge(("log_level", logging_string))
        .merge(("cli_colors", false))
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
    let conf = Settings::new().unwrap();

    let mut logging_level = LevelFilter::Error;
    if &conf.general.debug == &true{
        logging_level = LevelFilter::Debug;
    }

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
            .appender("logfile")
            .build(logging_level)).unwrap();

    log4rs::init_config(config);

    // This will run the necessary migrations.
    let connection = PgConnection::establish(conf.database.connection_string.as_str()).unwrap();
    embedded_migrations::run(&connection);


    create_routes(conf, cache::Cache{all_apps: vec![]}).await;
}