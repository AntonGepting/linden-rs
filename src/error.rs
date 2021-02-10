//use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    OsString(std::ffi::OsString),
    Log4rs(log4rs::config::runtime::ConfigErrors),
    Log(log::SetLoggerError),
    Custom(String),
    SerdeYaml(serde_yaml::Error),
}

impl Error {
    pub fn new(err: String) -> Self {
        Error::Custom(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(err: std::ffi::OsString) -> Self {
        Error::OsString(err)
    }
}

impl From<log4rs::config::runtime::ConfigErrors> for Error {
    fn from(err: log4rs::config::runtime::ConfigErrors) -> Self {
        Error::Log4rs(err)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::Log(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::SerdeYaml(err)
    }
}

//impl fmt::Display for Error {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//write!(f, "{}", self.err_text)
//}
//}
