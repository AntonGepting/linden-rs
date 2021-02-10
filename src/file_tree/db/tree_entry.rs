// XXX: BTreeMap - use first_entry() (currently nightly)
// XXX: rename to Node
// XXX: OsString as struct name
// XXX: regex | glob

use crate::file_tree::{Node, TreeEntryBody};
use std::collections::BTreeMap;

// XXX: mb BTreeMap as file field? avoiding multiple repeating field name
// XXX: rename file into name?
// XXX: type OsString?
// BTreeMap(name, body {desc, type...})
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeEntry(pub BTreeMap<String, Option<TreeEntryBody>>);

// example:
//
// children:
//  Vec<BTreeMap<name, body {desc, type, children}>>
//
// yaml output
//
// dir_name1:
//   desc: description
//   type: directory
//   children:
//     - file_name2:
//         desc: description
//         type: file
//
//     - file_name3:
//         desc: description
//         type: file
//
//
// extraction is like:
// if let Some(name, body) = self.first {
// }
//
impl TreeEntry {
    pub fn new(name: &str, body: TreeEntryBody) -> Self {
        let mut entry = BTreeMap::new();
        entry.insert(name.to_string(), Some(body));
        TreeEntry(entry)
    }

    // XXX: use nightly-only experimental API instead
    // XXX: return &T
    // get first (only first is needed, bc BTreeMap)
    pub fn first(&self) -> Option<(&String, &Option<TreeEntryBody>)> {
        self.0.iter().next()
    }

    // XXX: use nightly-only experimental API instead
    // get first name (only first is needed, bc BTreeMap)
    // XXX: return &T
    pub fn first_name(&self) -> Option<String> {
        self.first().and_then(|(name, _)| Some(name.to_owned()))
    }

    // XXX: use nightly-only experimental API instead
    // get first mut (only first is needed, bc BTreeMap)
    pub fn first_mut(&mut self) -> Option<(&String, &mut Option<TreeEntryBody>)> {
        self.0.iter_mut().next()
    }

    // get first only body (only first is needed, bc BTreeMap)
    pub fn first_body(&self) -> Option<&TreeEntryBody> {
        self.first().and_then(|(_, body)| body.as_ref())
    }

    // get first only body (only first is needed, bc BTreeMap)
    pub fn first_body_mut(&mut self) -> Option<&mut TreeEntryBody> {
        self.first_mut().and_then(|(_, body)| body.as_mut())
    }
}

// TODO: From<Path> to Type dir, symlink, file
// create from DirEntry
impl From<&Node> for TreeEntry {
    fn from(node: &Node) -> Self {
        let body = TreeEntryBody::from(node);
        let name = match node.borrow().file.clone().into_string() {
            Ok(name) => name,
            Err(_) => "name_error".to_string(),
        };
        TreeEntry::new(&name, body)
    }
}
