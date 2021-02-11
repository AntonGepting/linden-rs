use super::tree_entry::TreeEntry;
use crate::file_tree::{FileType, Node};
//use chrono::offset::Utc;
//use chrono::DateTime;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeEntryBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    // XXX: tags not supported now
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    // XXX: type into file/dir fields mb.?
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub file_type: Option<FileType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compare: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreeEntry>>,
    // XXX: mb custom deserializer?
    // XXX: mb flags locally?
    //pub flags: Option<FlagsEn<String, RefCell<ParentString>>>
    //pub flags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

impl TreeEntryBody {}

// TODO: From<Path> to Type dir, symlink, file
// create from DirEntry
impl From<&Node> for TreeEntryBody {
    fn from(node: &Node) -> Self {
        let node = node.borrow();

        // convert children first
        let children = match &node.children {
            Some(nodes) => {
                let mut entry_children = Vec::new();
                for child in nodes {
                    entry_children.push(TreeEntry::from(child));
                }
                Some(entry_children)
            }
            None => None,
        };

        // convert other body fields
        TreeEntryBody {
            desc: node.desc.clone(),
            children: children,
            tags: node.tags.clone(),
            sha256: node.sha256.clone(),
            status: node.status,
            modified: node.modified.clone(),
            accessed: node.accessed.clone(),
            created: node.created.clone(),
            size: node.size,
            file_type: node.file_type.clone(),
            compare: node.compare,
            hidden: node.hidden,
            comment: node.comment.clone(),
        }
    }
}
