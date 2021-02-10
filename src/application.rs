use crate::config::Config;
use crate::Cli;
//use crate::mylog::Log;
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct Application {
    pub custom_cfg: Option<PathBuf>,
    pub config: Config,
    pub project_path: PathBuf,
    pub log_level_filter: Option<LevelFilter>,
}

impl Application {
    pub fn new() -> Self {
        Default::default()
    }

    //pub fn init(&self) {
    //let log_filename = "linden.log";
    //Log::create(log_filename, Some(LevelFilter::Info)).unwrap();
    //info!("{}: {}", "start logging", log_filename);
    //}

    pub fn start(self) {
        let _cfg = Config::load(None).unwrap();
        let cli = Cli::new(self);

        cli.process_matches().unwrap();
    }
}
