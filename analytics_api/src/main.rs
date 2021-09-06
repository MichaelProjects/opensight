//#![feature(proc_macro_hygiene, decl_macro)]
//#![feature(in_band_lifetimes)]
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
mod logs;

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
use rocket_sync_db_pools::rocket::Rocket;
use rocket::{Ignite, Build};
use rocket::response::status::Custom;
use rocket::fairing::AdHoc;

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

pub fn rocket_creator(conf: Settings) -> Rocket<Build> {
    rocket::custom(insert_conf_values(&conf))
        .attach(AnalyticsDB::fairing())
        .manage(conf)
        .mount("/analytic", routes![get_health, insert_entry, insert_application] )
}

embed_migrations!("./migrations/");

#[rocket::main]
async fn main(){
    let conf = Settings::new().unwrap();

    logs::init_logger(&conf);

    // This will run the necessary migrations.
    let connection = PgConnection::establish(conf.database.connection_string.as_str()).unwrap();
    embedded_migrations::run(&connection);


    rocket_creator(conf).launch().await;
}


#[cfg(test)]
mod test {
    use super::rocket_creator;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use crate::settings::Settings;

    #[test]
    async fn test_hello() {
        let conf = Settings::new().unwrap();
        let client = Client::new(rocket_creator(conf)).await.unwrap();
        let mut response = client.get("/").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}