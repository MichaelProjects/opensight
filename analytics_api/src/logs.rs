use log4rs::append::file::FileAppender;
use log::LevelFilter;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use crate::settings::Settings;

pub fn init_logger(conf: &Settings) {
    let mut logging_level = LevelFilter::Error;
    if &conf.general.debug == &true{
    logging_level = LevelFilter::Debug;
    }

    let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
    .build("output.log").unwrap();

    let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
    .appender("logfile")
    .build(logging_level)).unwrap();

    log4rs::init_config(config);
}