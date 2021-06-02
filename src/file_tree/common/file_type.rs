use std::fmt;
use std::str::FromStr;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = "directory")]
    Directory,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "symlink")]
    Symlink,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FileType::Directory => "Directory",
            FileType::File => "File",
            FileType::Symlink => "Symlink",
        };
        write!(f, "{}", s)?;
        Ok(())
    }
}

// XXX: Err type def
impl FromStr for FileType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "Directory" => Ok(FileType::Directory),
            "File" => Ok(FileType::File),
            "Symlink" => Ok(FileType::Symlink),
            _ => Err(()),
        }
    }
}
