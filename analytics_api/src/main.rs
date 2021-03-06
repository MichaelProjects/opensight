//#![feature(proc_macro_hygiene, decl_macro)]
//#![feature(in_band_lifetimes)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod endpoints;
mod analytics;
mod application;
mod dao;
mod db;
mod health;
mod logs;
mod schema;
mod settings;
mod response;
mod analyse;
mod daos;
mod cors;
mod time_helper;

use crate::db::*;
use crate::settings::Settings;
use cors::CORS;
use diesel::prelude::*;
use endpoints::handler::{get_health, insert_entry, update_session, get_sessions};
use endpoints::analyse_handler::{get_analyse_new_user, get_analyse_data, get_analyse_user, get_analyse_session_length, get_device_display, get_version_info};
use rocket::figment::Figment;
use rocket::Build;
use rocket_sync_db_pools::rocket::Rocket;

pub fn insert_conf_values(conf: &Settings) -> Figment {
    let mut logging_string = "critical";
    if &conf.general.debug == &true {
        logging_string = "debug";
    }
    rocket::Config::figment()
        .merge(("port", &conf.general.port))
        .merge(("address", &conf.general.address))
        .merge(("log_level", logging_string))
        .merge(("cli_colors", false))
        .merge((
            "databases.postgres_url.url",
            &conf.database.connection_string,
        ))
}

pub fn rocket_creator(conf: Settings) -> Rocket<Build> {
    rocket::custom(insert_conf_values(&conf))
        .attach(AnalyticsDB::fairing())
        .attach(CORS)
        .manage(conf)
        .mount(
            "/analytic/v1",
            routes![insert_entry, update_session, get_health, get_sessions, get_analyse_new_user, get_analyse_data, get_analyse_user, get_analyse_session_length, get_device_display, get_version_info],
        )
}

embed_migrations!("./migrations/");

fn run_migration(conf: &Settings) {
    let connection = match PgConnection::establish(conf.database.connection_string.as_str()) {
        Ok(conn) => conn,
        Err(err) => panic!("Could not connect to Database, Postgres-Error: {}", err),
    };
    match embedded_migrations::run(&connection) {
        Ok(result) => result,
        Err(err) => panic!("Cloud not migrate Database Tables, error: {}", err),
    };
}


#[rocket::main]
async fn main() {
    let conf = match Settings::new() {
        Ok(conf) => conf,
        Err(_err) => panic!("Cloud not read Config, ensure it in the right place"),
    };
    logs::init_logger(&conf);
    run_migration(&conf);
    rocket_creator(conf).launch().await;
}

#[cfg(test)]
mod test {}
