#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod logs;
mod settings;
use rocket::{figment::Figment, Rocket};
use crate::settings::Settings;
use rocket::Build;


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
        .manage(conf)
        .mount(
            "/analytic",
            routes![],
        )
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
    logs::init_logger(&conf);
    rocket_creator(conf).launch().await;
}