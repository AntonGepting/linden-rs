#[macro_use]
extern crate log;
extern crate log4rs;

#[macro_use]
extern crate serde_derive;

extern crate text_tree_elements;

mod file_tree;

mod application;
mod cli;
mod config;
//mod db;
mod error;
mod mylog;

pub use crate::cli::cli::Cli;

use application::Application;
use std::io;

fn main() -> Result<(), io::Error> {
    let app = Application::new();
    //app.init();
    app.start();
    Ok(())
}
