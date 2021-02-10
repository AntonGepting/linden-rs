use crate::file_tree::{Error, Node, TreeEntry};
use std::fs;
use std::path::Path;

// XXX: mb merge cfg with entry structs, so entries have cfg borrowing from parents
// XXX: skip_serializing_null
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTreeCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub file_type: Option<bool>,
}

// XXX: skip_serializing_null
// XXX: rename
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfg: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FileTreeCfg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree: Option<TreeEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicts: Option<TreeEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<TreeEntry>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub fields: Option<TreeEntryFields>,
    // XXX: not implemented
    //pub flags: Option<Vec<String>>,
}

//pub struct TreeEntryFields(usize)

impl FileTree {
    // new from given tree and ignore list
    pub fn new(tree: Option<TreeEntry>, ignore: Option<Vec<String>>) -> Self {
        FileTree {
            ignore: ignore,
            tree: tree,
            ..Default::default()
        }
    }

    // new form node and ignore list
    pub fn create_from_node(node: &Node, ignore: Option<Vec<String>>) -> Self {
        let entry = TreeEntry::from(node);
        FileTree::new(Some(entry), ignore)
    }

    // read from yml
    pub fn read<P: AsRef<Path>>(file_path: P) -> Result<Self, Error> {
        let tree_str = fs::read_to_string(file_path)?;
        let file_tree: FileTree = serde_yaml::from_str(&tree_str)?;
        Ok(file_tree)
    }

    // write to yml
    pub fn write<P: AsRef<Path>>(self, file_path: P) -> Result<(), Error> {
        let tree_str = serde_yaml::to_string(&self)?;
        fs::write(file_path, tree_str.as_bytes())?;
        Ok(())
    }
}

// NOTE: too many arguments for trait implementation
//impl From<&Node> for FileTree {
//fn from(node: &Node) -> Self {
//}
//}
