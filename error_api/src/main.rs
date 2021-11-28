mod logs;
mod settings;
mod handler;

#[macro_use] extern crate rocket;

use diesel::prelude::*;
use rocket::figment::Figment;
use rocket_sync_db_pools::rocket::Rocket;
use rocket::{Build};
use crate::settings::{Settings};

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
        .manage(conf)

        .mount("/error", routes![] )
}

//embed_migrations!("./migrations/");

/*fn run_migration(conf: &Settings){
    let connection = match PgConnection::establish(conf.database.connection_string.as_str()){
        Ok(conn) => conn,
        Err(err) => panic!("Could not connect to Database, Postgres-Error: {}", err)
    };
    match embedded_migrations::run(&connection) {
        Ok(result) => result,
        Err(err) => panic!("Cloud not migrate Database Tables, error: {}", err)
    };
}*/

#[rocket::main]
async fn main(){

    let conf = match Settings::new(){
        Ok(conf) => conf,
        Err(_err) => panic!("Cloud not read Config, ensure it in the right place")
    };
    let _a = logs::init_logger(&conf);
    //run_migration(&conf);
    rocket_creator(conf).launch().await;
}
