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
    #[serde(rename = "unknown")]
    Unknown,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            FileType::Directory => "Directory",
            FileType::File => "File",
            FileType::Symlink => "Symlink",
            FileType::Unknown => "Unknown",
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
            "Unknown" => Ok(FileType::Unknown),
            _ => Err(()),
        }
    }
}

impl From<std::fs::FileType> for FileType {
    fn from(t: std::fs::FileType) -> Self {
        if t.is_dir() {
            FileType::Directory
        } else if t.is_file() {
            FileType::File
        } else if t.is_symlink() {
            FileType::Symlink
        } else {
            FileType::Unknown
        }
    }
}
