use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    SerdeYaml(serde_yaml::Error),
    //Recurse,
    //OsString(std::ffi::Error),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::IO(ref err) => err.fmt(f),
            Self::SerdeYaml(ref err) => err.fmt(f),
            //Self::Recurse => write!("recurse"),
            //Self::OsString(ref err) => err.fmt(f),
            Self::Custom(ref err) => err.fmt(f),
        }
    }
}

//impl fmt::Display for Error {
//fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//match self {
//Self::Tmux(ref msg) => write!(f, "{}", msg),
//Self::IO(ref err) => err.fmt(f),
//Self::ParseInt(ref err) => err.fmt(f),
//Self::Parse(ref err) => err.fmt(f),
//_ => "".fmt(f),
//}
//}
//}

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

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::SerdeYaml(err)
    }
}

//impl From<std::ffi::Error> for Error {
//fn from(err: std::ffi::Error) -> Self {
//Error::OsString(err)
//}
//}
