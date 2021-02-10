use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
//use log4rs::config::{Appender, Config, Logger, Root};
use super::error::Error;
use log4rs::config::{Appender, Config, Root};
use log4rs::{init_config, Handle};

pub struct Log;

impl Log {
    const LOG_MESSAGE_FMT: &'static str = "{d(%d.%m.%Y %H:%M:%S)} {l} - {m}{n}";
    const LOG_APPENDER_FILE: &'static str = "logfile";
    //const LOG_APP_APPENDER_FILE: &'static str = "app::logfile";
    const LOG_APPENDER_STDOUT: &'static str = "stdout";

    pub fn create(filename: &str, level_filter: Option<LevelFilter>) -> Result<Handle, Error> {
        let lvl = level_filter.unwrap_or(LevelFilter::Error);

        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(Log::LOG_MESSAGE_FMT)))
            .build();

        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(Log::LOG_MESSAGE_FMT)))
            .build(filename)?;

        let log_config = Config::builder()
            .appender(Appender::builder().build(Log::LOG_APPENDER_STDOUT, Box::new(stdout)))
            .appender(Appender::builder().build(Log::LOG_APPENDER_FILE, Box::new(logfile)))
            //.logger(Logger::builder().appender(LOG_APPENDER_FILE).additive(false).build("app::logfile", LevelFilter::Debug))
            .build(
                Root::builder()
                    .appenders(vec![Log::LOG_APPENDER_STDOUT, Log::LOG_APPENDER_FILE])
                    .build(lvl),
            )?;

        Ok(init_config(log_config)?)
    }

    //static LOG_CLI_MESSAGE: &'static str = "CLI got command {}";
}
