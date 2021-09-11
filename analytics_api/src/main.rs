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
mod logs;

use crate::settings::{Settings};
use crate::db::*;
use handler::{get_health, insert_entry, insert_application, get_applications};
use diesel::prelude::*;
use rocket::figment::Figment;
use rocket_sync_db_pools::rocket::Rocket;
use rocket::{Build};


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
        .mount("/analytic", routes![get_health, insert_entry, insert_application, get_applications] )
}

embed_migrations!("./migrations/");

fn run_migration(conf: &Settings){
    let connection = match PgConnection::establish(conf.database.connection_string.as_str()){
        Ok(conn) => conn,
        Err(err) => panic!("Could not connect to Database, Postgres-Error: {}", err)
    };
    match embedded_migrations::run(&connection) {
        Ok(result) => result,
        Err(err) => panic!("Cloud not migrate Database Tables, error: {}", err)
    };
}

#[rocket::main]
async fn main(){
    let conf = match Settings::new(){
        Ok(conf) => conf,
        Err(err) => panic!("Cloud not read Config, ensure it in the right place")
    };
    logs::init_logger(&conf);
    run_migration(&conf);
    rocket_creator(conf).launch().await;
}


#[cfg(test)]
mod test {
    use super::rocket_creator;
    use rocket::http::Status;
    use crate::settings::Settings;
    use rocket::local::blocking::Client;

    #[test]
    fn test_health_check() {
        let conf = Settings::new().unwrap();
        let client = Client::tracked(rocket_creator(conf)).unwrap();
        let response = client.get("/analytic/health");
        println!("{:?}", response);
    }
}