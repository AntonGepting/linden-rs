use crate::application::Application;
use crate::cli::command::Command;
use crate::cli::common::*;
use crate::error::Error;
use crate::mylog::Log;
use clap::{App, ArgMatches};

use crate::file_tree::{FileTree, Node, NODE_DEFAULT, NODE_NONE, SORT_DSC};
use log::LevelFilter;
use std::fs;
use std::path::{Path, PathBuf};

//#[derive(Default, Debug)]
pub struct Cli<'a, 'b> {
    pub cli: App<'a, 'b>,
    pub app: Application,
}

impl<'a, 'b> Cli<'a, 'b> {
    // check if any command found
    pub fn process_matches(mut self) -> Result<(), Error> {
        // process keys
        let matches = self.cli.get_matches();

        // verbose level
        self.app.log_level_filter = if matches.is_present(KEY_QUIET) {
            // silent
            // NOTE: clap is still reporting errors
            Some(LevelFilter::Off)
        } else {
            // verbose level
            match matches.occurrences_of(KEY_VERBOSE) {
                0 => None,
                1 => Some(LevelFilter::Error),
                2 => Some(LevelFilter::Warn),
                3 => Some(LevelFilter::Info),
                4 => Some(LevelFilter::Debug),
                5 => Some(LevelFilter::Trace),
                // XXX: print too many -v flags
                _ => Some(LevelFilter::Trace),
            }
        };

        // logging on?
        if matches.is_present(KEY_LOG) {
            let log_filename = matches.value_of(KEY_LOG).unwrap_or(DEFAULT_LOG_FILENAME);
            Log::create(log_filename, self.app.log_level_filter).unwrap();
            info!("{}: {}", "start logging", log_filename);
        }

        error!("error example");
        warn!("warn example");
        info!("info example");
        debug!("debug example");
        trace!("trace example");

        // load custom / create default app config
        if matches.is_present(KEY_CFG) {
            self.app.custom_cfg = matches.value_of(KEY_CFG).map(|s| PathBuf::from(s));
            //match Config::load(self.app.custom_cfg.as_ref()) {
            //Ok(config) => self.app.config = config,
            //Err(e) => error!("{}: {}", LOG_CLI_CFG_ERROR, e.message),
        };

        let mut cmd = Command::new();

        // commands
        cmd.match_command(&matches);

        // exit msg
        info!("{}", LOG_CLI_END);

        Ok(())
    }
}
