use crate::settings::Settings;
use log::{LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};

pub fn init_logger(conf: &Settings){
    let mut logging_level = LevelFilter::Error;
    if &conf.general.debug == &true {
        logging_level = LevelFilter::Debug;
    }
    if &conf.general.log_file != "" {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("/var/log/opensight/analytics_api.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(logging_level))
            .unwrap();
        log4rs::init_config(config).expect("Could not set logging file path");
    } else {
        env_logger::init();
    }
}
