//! logging.rs
//! handles sonic logging

use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

pub fn log_init() -> Handle {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
        )))
        .build();

    // Create a file appender
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
        )))
        .build("log/requests.log")
        .expect("Failed to create file appender");

    // Build the log4rs configuration
    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", log::LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", log::LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("requests")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    // use handle to change logger configuration at runtime
    let _handle = log4rs::init_config(config).expect("Failed to initialize log4rs");

    // Set the log4rs handle as the global logger
    log::set_max_level(log::LevelFilter::Debug);
    return _handle;
}
