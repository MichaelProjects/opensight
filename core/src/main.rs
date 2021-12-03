use rocket::figment::Figment;
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
        .attach(AnalyticsDB::fairing())
        .manage(conf)
        .mount(
            "/analytic",
            routes![insert_entry, update_session],
        )
        .mount(
            "/analytic/admin",
            routes![insert_application, get_applications, get_application_entrys],
        )
}