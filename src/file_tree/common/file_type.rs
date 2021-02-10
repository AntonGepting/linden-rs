use std::fmt;

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
