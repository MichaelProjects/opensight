#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod logs;
mod settings;
mod db;
mod handle;
mod health;
mod application;
mod schema;
mod application_dao;
mod user;
mod response;
mod user_dao;

use diesel::{PgConnection, Connection};
use rocket::{figment::Figment, Rocket};
use crate::settings::Settings;
use rocket::Build;
use handle::{get_health, create_application, get_application, get_all_application};
use db::DatabaseConnection;

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
        .attach(DatabaseConnection::fairing())    
        .manage(conf)
        .mount(
            "/core/v1",
            routes![get_health, create_application, get_application, get_all_application],
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
async fn main(){
    let conf = match Settings::new() {
        Ok(conf) => conf,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    run_migration(&conf);
    logs::init_logger(&conf);
    rocket_creator(conf).launch().await;
}
