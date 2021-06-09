// TODO: split in tree and node
//#![forbid(unsafe_code)]
//#![warn(missing_docs)]
use crate::cli::color::TerminalColor;
use crate::file_tree::common::constants::*;
use crate::file_tree::node::color_scheme::ColorScheme;
use crate::file_tree::{Error, FileTree, FileType, TreeEntry};
use chrono::offset::Utc;
use chrono::DateTime;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::DirEntry;
use std::path::{Component, Path, PathBuf};
use std::rc::{Rc, Weak};
use text_tree_elements::TextTreeElements;

/// default/child pointer (refcell)
pub type NodeRc = Rc<RefCell<NodeData>>;
/// parent pointer
pub type NodeWeak = Weak<RefCell<NodeData>>;

/// newtype wrapper
/// because methods needed for this type (for NodeRc impossible)
#[derive(Debug, Clone, Default)]
pub struct Node(pub NodeRc);

// XXX: mb ignore in every node?
/// main structure holding data
///  NodeRc is needed to avoid recursive pointers
#[derive(Debug, Clone, Default)]
pub struct NodeData {
    /// file/directory/symlink name
    pub file: OsString,
    // XXX: mb custom deserializer?
    pub parent: Option<NodeWeak>,
    /// children nodes, if it's a directory
    pub children: Option<Vec<Node>>,
    /// description
    pub desc: Option<String>,
    // XXX: tags not supported now
    /// tags based on a program name, type, contents
    pub tags: Option<Vec<String>>,
    /// sha256 hash
    pub sha256: Option<String>,
    /// comparation status
    pub status: Option<usize>,
    /// last modification date time
    pub modified: Option<String>,
    /// last access date time
    pub accessed: Option<String>,
    /// creation date time
    pub created: Option<String>,
    /// file size
    pub size: Option<u64>,
    // XXX: type into file/dir fields mb.?
    /// file, directory, symlink
    pub file_type: Option<FileType>,
    /// ?
    pub compare: Option<usize>,
    // XXX: mb flags locally?
    //pub flags: Option<FlagsEn<String, RefCell<ParentString>>>
    //pub flags: Option<Vec<String>>,
    /// hide node from the standard filter output
    pub hidden: Option<bool>,
    /// user's comment
    pub comment: Option<String>,
}

impl NodeData {}

// XXX: think about return same type for all fn's returning Node, then maybe unwrap Rc? Same work
// for all such functions
impl Node {
    /// add child, create children vec if not exist (for first child)
    pub fn add_child(&self, child: NodeRc) {
        // save parent
        child.borrow_mut().parent = Some(Rc::downgrade(&self.0));

        // create vec if not exists, and append
        let child = Node(child);
        if self.has_children() {
            self.borrow_mut()
                .children
                .as_mut()
                .and_then(|children| Some(children.push(child)));
        } else {
            self.borrow_mut().children = Some(vec![child])
        }
    }

    /// get weak pointer
    pub fn to_weak(node: &Node) -> NodeWeak {
        Rc::downgrade(&node.0)
    }

    /// add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: empty file field for root directory
    pub fn add_path<P: AsRef<Path>>(
        &mut self,
        parent: Option<NodeWeak>,
        dir: P,
        ignore: Option<&Vec<String>>,
    ) -> Result<(), Error> {
        self.add_path_ext(parent, dir, ignore, NODE_DEFAULT)
    }

    // add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: empty file field for root directory
    // XXX: add with and without children
    // XXX: add root . directory fields
    // XXX: check existance and accept if exist path
    pub fn add_path_ext<P: AsRef<Path>>(
        &mut self,
        parent: Option<NodeWeak>,
        dir: P,
        ignore: Option<&Vec<String>>,
        bitflag: usize,
    ) -> Result<(), Error> {
        // XXX: absolete path
        //self.file = fs::canonicalize(&dir).unwrap().to_str().unwrap().to_string();

        // XXX: sorted_read_dir()
        for read_dir in fs::read_dir(dir)? {
            let dir_entry = read_dir?;

            // skip if in ignore list, mb list paths?
            if Self::is_ignore(&dir_entry, ignore) {
                continue;
            }

            // create and fill entry
            let node_rc = Node::from_dir_entry_ext(parent.clone(), &dir_entry, bitflag);
            let node_weak = Rc::downgrade(&node_rc);

            //let node = node_rc.borrow()
            let path = dir_entry.path();

            // if dir => process it's children before save node
            if path.is_dir() {
                Node(Rc::clone(&node_rc)).add_path_ext(Some(node_weak), &path, ignore, bitflag)?;
            }

            self.add_child(node_rc);
        }
        Ok(())
    }

    // is DirEntry in ignore list
    pub fn is_ignore(dir_entry: &DirEntry, ignore: Option<&Vec<String>>) -> bool {
        // skip if in ignore list, mb list paths?
        let name = dir_entry.file_name().into_string().unwrap();
        if let Some(ref ignore_vec) = ignore {
            if ignore_vec.contains(&name) {
                return true;
            }
        }
        false
    }

    //pub fn merge_preview<P: AsRef<Path>>(

    // merge curretn file tree with dir
    //  add not existing nodes, set state
    //  set state for existing
    //
    // XXX: not from file, but from struct is better?
    pub fn fill_compare_status<P: AsRef<Path>>(
        &mut self,
        parent: Option<NodeWeak>,
        dir: P,
        ignore: Option<&Vec<String>>,
        bitflag: usize,
    ) -> Result<(), Error> {
        // XXX: absolete path
        //self.file = fs::canonicalize(&dir).unwrap().to_str().unwrap().to_string();

        // XXX: sorted_read_dir()
        for read_dir in fs::read_dir(dir)? {
            let dir_entry = read_dir?;

            // exists in ignore list
            if Self::is_ignore(&dir_entry, ignore) {
                continue;
            }

            // create and fill entry
            let node_rc = Node::from_dir_entry_ext(parent.clone(), &dir_entry, bitflag);
            let node_weak = Rc::downgrade(&node_rc);

            //let node = node_rc.borrow()
            let path = dir_entry.path();

            // if file already exists in db
            if let Some(mut node) = self.get_child(dir_entry.file_name()) {
                // if dir => process it's children before save node
                if path.is_dir() {
                    //let node_data = node.borrow();
                    node.fill_compare_status(parent.clone(), &path, ignore, bitflag)?;
                    // dir and not exists
                }

                node.borrow_mut().status = Some(Node(node_rc).compare_ext(&node, bitflag));

            // if not exists in db
            } else {
                if path.is_dir() {
                    Node(Rc::clone(&node_rc)).fill_compare_status(
                        Some(node_weak),
                        &path,
                        ignore,
                        bitflag,
                    )?;
                }

                node_rc.borrow_mut().status = Some(NODE_UNTRACKED);
                self.add_child(node_rc);
            }
        }

        Ok(())
    }

    // XXX: string as output
    pub fn bitflag_to_string(bitflag: usize) -> String {
        let mut v = Vec::new();
        if (bitflag & NODE_DESC) > 0 {
            v.push("desc");
        }
        if (bitflag & NODE_SHA256) > 0 {
            v.push("sha256");
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            v.push("modified");
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            v.push("accessed");
        }
        if (bitflag & NODE_CREATED) > 0 {
            v.push("created");
        }
        if (bitflag & NODE_SIZE) > 0 {
            v.push("size");
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            v.push("type");
        }
        if (bitflag & NODE_TAGS) > 0 {
            v.push("tags");
        }
        if (bitflag & NODE_COMMENT) > 0 {
            v.push("comment");
        }
        v.join(" ")
    }

    // clear given fields, recursive if needed
    pub fn clear_ext(&self, bitflag: usize, recursive: bool) {
        // NOTE: can't clear name, only remove entry
        if (bitflag & NODE_DESC) > 0 {
            self.borrow_mut().desc = None;
        }
        if (bitflag & NODE_SHA256) > 0 {
            self.borrow_mut().sha256 = None;
        }
        if (bitflag & NODE_STATUS) > 0 {
            self.borrow_mut().status = None;
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            self.borrow_mut().modified = None;
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            self.borrow_mut().accessed = None;
        }
        if (bitflag & NODE_CREATED) > 0 {
            self.borrow_mut().created = None;
        }
        if (bitflag & NODE_SIZE) > 0 {
            self.borrow_mut().size = None;
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            self.borrow_mut().file_type = None;
        }
        if (bitflag & NODE_TAGS) > 0 {
            self.borrow_mut().tags = None;
        }
        // XXX: check, drop needed? recursive anyway
        if (bitflag & NODE_CHILDREN) > 0 {
            self.borrow_mut().children = None;
        }
        // for children too
        if recursive {
            if let Some(children) = &mut self.borrow_mut().children {
                for child in children {
                    child.clear_ext(bitflag, recursive);
                }
            }
        }
    }

    // XXX: ugly, need to be rewritten
    // XXX: bitflags or enum? mb bitflags, coz asc desc etc is possible too
    // XXX: all fields? one after another, checking e.g. extension then creation time
    // XXX: match or if statement
    pub fn cmp_ext(&self, origin: &Node, bitflag: usize) -> Ordering {
        //if bitflag & COMPARE_NAME > 0 {
        //result = name.cmp(&orig_name)
        //}
        let node = self.borrow();
        let origin = origin.borrow();

        let mut result = match bitflag & NODE_ALL {
            NODE_NAME => node.file.cmp(&origin.file),
            NODE_DESC => node.desc.cmp(&origin.desc),
            NODE_TAGS => node.tags.cmp(&origin.tags),
            NODE_SHA256 => node.sha256.cmp(&origin.sha256),
            NODE_STATUS => node.status.cmp(&origin.status),
            NODE_MODIFIED => node.modified.cmp(&origin.modified),
            NODE_ACCESSED => node.accessed.cmp(&origin.accessed),
            NODE_CREATED => node.created.cmp(&origin.created),
            NODE_SIZE => node.size.cmp(&origin.size),
            NODE_FILE_TYPE => node.file_type.cmp(&origin.file_type),
            _ => Ordering::Less,
        };
        if (bitflag & SORT_ASC) > 0 {
            result = result.reverse();
        }
        result
    }

    // compare current entry with origin using bitflags
    // true - same file
    // false - differs
    // XXX: add default as argument?
    // XXX: add heuristic compare fn
    // XXX: tags
    pub fn compare(&self, origin: &Node) -> usize {
        self.compare_ext(origin, NODE_DEFAULT)
    }

    // XXX: None == None?
    // XXX: time older younger?
    // XXX: tags
    //
    /// compare selected fields of two nodes and return bitflag of differences
    /// compare nodes, returns what fields are different
    ///
    /// return: NODE_NODE - no changes
    ///         NODE_* - detected changes
    pub fn compare_ext(&self, origin: &Node, bitflag: usize) -> usize {
        let mut result = NODE_NONE;

        let node = self.borrow();
        let origin = origin.borrow();

        if (bitflag & NODE_NAME) > 0 {
            if node.file != origin.file {
                result |= NODE_NAME;
            }
        }
        if (bitflag & NODE_DESC) > 0 {
            if node.desc != origin.desc {
                result |= NODE_DESC;
            }
        }
        if (bitflag & NODE_TAGS) > 0 {
            if node.tags != origin.tags {
                result |= NODE_TAGS;
            }
        }
        if (bitflag & NODE_SHA256) > 0 {
            if node.sha256 != origin.sha256 {
                result |= NODE_SHA256;
            }
        }
        if (bitflag & NODE_STATUS) > 0 {
            if node.status != origin.status {
                result |= NODE_STATUS;
            }
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            if node.modified != origin.modified {
                result |= NODE_MODIFIED;
            }
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            if node.accessed != origin.accessed {
                result |= NODE_ACCESSED;
            }
        }
        if (bitflag & NODE_CREATED) > 0 {
            if node.created != origin.created {
                result |= NODE_CREATED;
            }
        }
        if (bitflag & NODE_SIZE) > 0 {
            if node.size != origin.size {
                result |= NODE_SIZE;
            }
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            if node.file_type != origin.file_type {
                result |= NODE_FILE_TYPE;
            }
        }

        result
    }

    /// copy selected fields from origin
    pub fn copy_ext(&self, origin: &Node, bitflag: usize) {
        let mut node = self.borrow_mut();
        let origin = origin.borrow();

        if (bitflag & NODE_NAME) > 0 {
            node.file = origin.file.clone();
        }
        if (bitflag & NODE_DESC) > 0 {
            node.file = origin.file.clone();
        }
        if (bitflag & NODE_TAGS) > 0 {
            node.tags = origin.tags.clone();
        }
        if (bitflag & NODE_SHA256) > 0 {
            node.sha256 = origin.sha256.clone();
        }
        if (bitflag & NODE_STATUS) > 0 {
            node.status = origin.status;
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            node.modified = origin.modified.clone();
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            node.accessed = origin.accessed.clone();
        }
        if (bitflag & NODE_CREATED) > 0 {
            node.created = origin.created.clone();
        }
        if (bitflag & NODE_SIZE) > 0 {
            node.size = origin.size.clone();
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            node.file_type = origin.file_type.clone();
        }
    }

    // XXX: -1?
    pub fn children_num(&self) -> usize {
        match &self.0.borrow().children {
            Some(children) => children.len(),
            None => 0,
        }
    }

    /// create node with user given data
    // XXX: create paths force?
    // TODO: default path
    pub fn create<P: AsRef<Path> + Copy>(&mut self, path: P, mut data: NodeData) -> Option<()> {
        // get dir path from full path, without file
        let dir_name = path.as_ref().parent().unwrap_or(Path::new("."));
        // does parent node exist?
        self.get(dir_name).and_then(|parent_node| {
            // get file name
            path.as_ref().file_name().and_then(|file_name| {
                // if child not exists
                if parent_node.get_child(file_name).is_none() {
                    data.file = file_name.to_owned();
                    data.parent = Some(Rc::downgrade(&parent_node.0));
                    let node = Rc::new(RefCell::new(data));
                    self.add_child(node);
                    Some(())
                } else {
                    None
                }
            })
        })
    }

    // add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: overwrites pure file name of the root directory with full path
    pub fn create_from_path<P: AsRef<Path>>(
        dir: P,
        ignore: Option<&Vec<String>>,
    ) -> Result<Node, Error> {
        Node::create_from_path_ext(dir.as_ref(), ignore, NODE_DEFAULT)
    }

    // add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: overwrites pure file name of the root directory with full path
    pub fn create_from_path_ext<P: AsRef<Path>>(
        dir: P,
        ignore: Option<&Vec<String>>,
        bitflag: usize,
    ) -> Result<Node, Error> {
        let file = dir.as_ref().as_os_str();
        let mut node = Node(Node::new(file.to_owned(), None));

        node.add_path_ext(None, dir, ignore, bitflag)?;
        Ok(node)
    }

    pub fn desc<S: Into<String>>(&self, desc: Option<S>) {
        self.borrow_mut().desc = desc.map(|s| s.into());
    }

    /// check if given path exist in node tree
    pub fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        match self.get(path) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn comment<S: Into<String>>(&self, comment: Option<S>) {
        self.borrow_mut().comment = comment.map(|s| s.into());
    }

    pub fn hidden(&self, hidden: Option<bool>) {
        self.borrow_mut().hidden = hidden;
    }

    /// init fields
    //pub fn init(&self, desc: Option<String>, comment: Option<String>) {
    //let mut node = self.borrow_mut();
    //node.desc = desc;
    //node.comment = comment;
    //}

    pub fn find(&self, name: &str) {
        //if let Some(name) = name {
        //if let Some(children) = root.get_child(name) {
        //root = children;
        //} else {
        //return None;
        //}
        //}
        if let Some(children) = &self.borrow().children {
            for child in children {
                child.find(name);
            }
        }
        unimplemented!();
    }

    pub fn for_all(
        &self,
        level: usize,
        current_idx: usize,
        size: usize,
        cb: &dyn Fn(&Self, usize, usize, usize),
    ) {
        cb(&self, level, current_idx, size);
        if let Some(children) = &self.0.borrow().children {
            for (i, child) in children.iter().enumerate() {
                // XXX ?
                child.for_all(level + 1, i, children.len(), cb);
            }
        }
    }

    //pub fn for_all2(
    //&self,
    //level: usize,
    //current_idx: usize,
    //size: usize,
    //path
    //cb: &dyn Fn(&Self, usize, usize, usize),
    //) {
    //cb(&self, level, current_idx, size);
    //if let Some(children) = &self.0.borrow().children {
    //for (i, child) in children.iter().enumerate() {
    //// XXX ?
    //child.for_all(level + 1, i, children.len(), cb);
    //}
    //}
    //}

    pub fn sort_ext(&self, bitflag: usize) {
        if let Some(ref mut children) = self.borrow_mut().children {
            children.sort_by(|a, b| a.cmp_ext(b, bitflag));
            children
                .iter_mut()
                .for_each(|child| child.sort_ext(bitflag));
        }
    }

    // call cb for all children of an entry
    pub fn for_children(&self, path: &Path, cb: &dyn Fn(&Node, usize, usize)) -> Result<(), Error> {
        if let Some(entry) = self.get(path) {
            if let Some(children) = &entry.borrow().children {
                let size = children.len();
                for (i, child) in children.iter().enumerate() {
                    cb(&child, i, size);
                }
                Ok(())
            } else {
                Err(Error::new(String::from("no children exist")))
            }
        } else {
            Err(Error::new(String::from("file or path not tracked")))
        }
    }

    // FIXME: in development
    //pub fn merge(&self, origin: &Node) {
    //// get full path as id
    //let path = self.get_full_path();

    //let node = self.borrow();
    ////let mut s: String = node.file.to_string();

    //// if found by filename in old tree
    ////if let Some(origin_entry) = origin.remove(&path) {
    //if let Some(origin_entry) = origin.get(&path) {
    //s = format!("{} (exists)", s);
    //// compare old entry with new one using bitflags
    //let cmp_result = self.compare_ext(&origin_entry);
    //// show status
    //if cmp_result != COMPARE_NONE {
    //// update selected fields
    //self.update_ext(&origin_entry, COMPARE_DESC | COMPARE_TAGS);
    //s = format!("{} changes: {}", s, Node::bitflag_to_string(cmp_result));
    //} else {
    //s = format!("{} changes: no", s);
    //}
    //// XXX: not found by filename, try to use kind of heuristics
    //// XXX: move to removed
    //} else {
    //s = format!("{} (new)", s);
    ////self.0.remove
    //}
    //println!("{}", s);

    //let node = self.borrow_mut();
    //// check all entries in own tree
    //if let Some(children) = &mut node.children {
    //for child in children {
    //child.merge(&origin);
    //}
    //}
    ////}
    //}

    pub fn from_dir_entry_ext(
        parent: Option<NodeWeak>,
        dir_entry: &DirEntry,
        bitflag: usize,
    ) -> NodeRc {
        // create empty entry
        let node_rc = Node::new(dir_entry.file_name(), parent);
        let mut node = node_rc.borrow_mut();
        //println!("{:?}", &dir_entry.file_name());
        node.file = dir_entry.file_name();
        //node.path = dir_entry.path();

        if (bitflag & NODE_DESC) > 0 {
            node.desc = Some(String::from(""));
        }

        // metadata exists? save .created .acessed .modified .len
        if let Ok(metadata) = dir_entry.metadata() {
            if (bitflag & NODE_CREATED) > 0 {
                if let Ok(created) = metadata.created() {
                    node.created = Some(DateTime::<Utc>::from(created).to_string());
                }
            }
            if (bitflag & NODE_ACCESSED) > 0 {
                if let Ok(accessed) = metadata.accessed() {
                    node.accessed = Some(DateTime::<Utc>::from(accessed).to_string());
                }
            }
            if (bitflag & NODE_MODIFIED) > 0 {
                if let Ok(modified) = metadata.modified() {
                    node.modified = Some(DateTime::<Utc>::from(modified).to_string());
                }
            }
            if (bitflag & NODE_SIZE) > 0 {
                node.size = Some(metadata.len());
            }

            if (bitflag & NODE_FILE_TYPE) > 0 {
                // file_type
                let file_type = metadata.file_type();
                if file_type.is_dir() {
                    node.file_type = Some(FileType::Directory);
                } else if file_type.is_file() {
                    node.file_type = Some(FileType::File);
                } else if file_type.is_symlink() {
                    node.file_type = Some(FileType::Symlink);
                }
            }

            if (bitflag & NODE_TAGS) > 0 {
                node.tags = Some(vec![String::from("")]);
            }
            //permissions
        }

        if (bitflag & NODE_COMMENT) > 0 {
            node.desc = Some(String::from(""));
        }

        // calc SHA256 hash for file
        //let path = dir_entry.path();
        //if path.is_file() {
        //tree.sha256 = Some(get_sha256(&path).unwrap());
        //}
        node_rc.clone()
    }

    // get entry by path
    // XXX: ugly
    // XXX: implement root . ls
    pub fn get<P: AsRef<Path>>(&self, path: P) -> Option<Node> {
        // current root of the path (changing by running through)
        let mut root = self.to_owned();

        // if root .
        //if path.as_ref() == Path::new(".") {
        //return Some(root);
        //}

        // iterate over path elements
        for component in path.as_ref().components() {
            //println!("{:?} {:?}", Component::Normal(name), &component);
            // skip root . (accept ./foo/bar/file.ext foo/bar/file.ext)
            // XXX: mb just remove root . before calling get from path?
            if component == Component::CurDir {
                continue;
            }
            if let Component::Normal(name) = component {
                match root.get_child(name) {
                    Some(node) => root = node,
                    None => return None,
                }
            } else {
                return None;
            }
        }
        // full loop done, path found, return
        return Some(root);
    }

    // get child by name if exists
    // XXX: what with entry body? only children affected
    pub fn get_child<S: AsRef<OsStr>>(&self, file: S) -> Option<Node> {
        // OPT: self.children.and_then(|children| children.iter)
        if let Some(children) = &self.borrow().children {
            for child in children {
                if child.borrow().file == file.as_ref() {
                    return Some(child.clone());
                }
            }
        }
        None
    }

    pub fn get_child_idx<S: AsRef<OsStr>>(&self, file: S) -> Option<usize> {
        self.borrow().children.as_ref().and_then(|children| {
            children
                .iter()
                .position(|child| child.borrow().file == file.as_ref())
        })
    }

    // TODO: write it
    // XXX: Option
    // XXX: examples check is it properly
    // XXX: destroy vec if last aswell
    pub fn remove<P: AsRef<Path>>(&self, path: P) -> Option<()> {
        // get parent node
        if let Some(file) = path.as_ref().file_name() {
            if let Some(dir) = path.as_ref().parent() {
                if let Some(node) = self.get(dir) {
                    node.remove_child(file);
                }
            } else {
                self.remove_child(file);
            }
        }

        Some(())
    }

    // remove child by name
    // XXX: if last set children None
    pub fn remove_child<S: AsRef<OsStr>>(&self, file: S) {
        self.borrow_mut().children.as_mut().and_then(|children| {
            Some(children.retain(|child| child.borrow().file != file.as_ref()))
        });
    }

    /// borrow() wrapper
    pub fn borrow(&self) -> Ref<NodeData> {
        self.0.borrow()
    }

    /// borrow_mut() wrapper
    pub fn borrow_mut(&self) -> RefMut<NodeData> {
        self.0.borrow_mut()
    }

    /// get full path and file name of entry
    pub fn get_full_path(this: &NodeData) -> PathBuf {
        let mut path = Node::get_path(&this);
        path.push(&this.file);
        path
    }

    pub fn get_parent(&self) -> Option<NodeRc> {
        self.0.borrow().parent.as_ref().and_then(Weak::upgrade)
    }

    /// get parent node of parent path from full path
    /// (example: `/home/user/example/file.rs` -> `/home/user/example`)
    pub fn get_parent_node_from_full_path<P: AsRef<Path>>(&self, full_path: P) -> Option<Node> {
        self.get_parent_node_from_full_path_ext(full_path, ".")
    }

    /// get parent node of parent path from full path
    /// (example: `/home/user/example/file.rs` -> `/home/user/example`)
    /// (example: `./file.rs` -> `[default]`)
    pub fn get_parent_node_from_full_path_ext<P: AsRef<Path>, Q: AsRef<Path>>(
        &self,
        full_path: P,
        default: Q,
    ) -> Option<Node> {
        let dir = full_path.as_ref().parent().unwrap_or(default.as_ref());
        self.get(dir)
    }

    // another way?
    pub fn get_parent_by_name<P: AsRef<Path>>(&self, path: P) -> Option<NodeRc> {
        match self.get(path) {
            Some(node) => match node.borrow().parent.as_ref() {
                Some(parent) => parent.upgrade(),
                None => None,
            },
            None => None,
        }
        // compile time error:
        //self.get(path)
        //.and_then(|node| node.borrow().parent.as_ref())
        //.and_then(|parent| parent.upgrade())
    }

    // XXX: follow symlinks?
    /// get full path using parents fields excluding file name for given entry
    pub fn get_path(this: &NodeData) -> PathBuf {
        let mut path = PathBuf::new();
        if let Some(parent) = &this.parent {
            if let Some(parent) = parent.upgrade() {
                path = PathBuf::from(Node::get_path(&parent.borrow()));
                path.push(&parent.borrow().file);
            }
        }
        path
    }

    pub fn has_children(&self) -> bool {
        self.borrow().children.is_some()
    }

    pub fn has_child(&self, file: &OsStr) -> bool {
        if let Some(children) = &self.borrow().children {
            for child in children {
                if child.borrow().file == file {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub fn ls(&self, path: &Path) -> Result<(), Error> {
        let s = self.to_string_ext(NODE_DEFAULT);
        println!("{}", s);
        //let curr_path = Node::get_full_path(&self.borrow());
        //let curr_path_str = curr_path.to_str().unwrap();
        self.for_children(path, &|child, _i, _size| {
            //let s = child.to_string_ext(COMPARE_DEFAULT);
            //println!("{}", s);
            child.ls(path).unwrap_or(());
        })
    }

    pub fn new<S: Into<OsString>>(file: S, parent: Option<NodeWeak>) -> NodeRc {
        Rc::new(RefCell::new(NodeData {
            file: file.into(),
            parent,
            ..Default::default()
        }))
    }

    pub fn to_node(node: NodeRc) -> Node {
        Node(node)
    }

    // fill parent field for all tree
    pub fn print_for_all(&self) {
        self.for_all(0, 0, self.children_num(), &|entry, level, idx, size| {
            let s = entry.to_string_ext(NODE_NAME | NODE_DESC);
            //let (tab, link) =
            //TextTree::get_tabs(level, idx, size, "", ["", "│  ", "   "], ["", "├─ ", "└─ "]);

            //println!("{}{}{}", tab, link, s);
            //println!("{} {} {} {:?} {:?}", level, idx, size, name, body.size);
        });
    }

    // no tree, in line
    pub fn print_in_line(&self) {
        self.for_all(0, 0, self.children_num(), &|entry, _, _, _| {
            let s = entry.to_string_ext(NODE_NAME | NODE_DESC);
            println!(
                "{} {}",
                Node::get_full_path(&entry.borrow())
                    .into_os_string()
                    .to_str()
                    .unwrap(),
                s
            );
        });
    }

    // XXX: mb move to tree? tree and origin tree
    /// find using giving template node (same path will be used), and compare nodes
    ///
    /// Returns:
    /// * `NODE_NOT_EXISTS` - if template node path not found in current node tree
    /// * `NODE_*` - comparsion result, bitmask of differences between them
    pub fn find_and_compare_ext(&self, template: &Node, bitflag: usize) -> usize {
        let path = Node::get_full_path(&template.borrow());
        if let Some(node) = &self.get(&path) {
            node.compare_ext(&template, bitflag)
        } else {
            NODE_NOT_EXISTS
        }
    }

    pub fn find_and_compare(&self, template: &Node) -> usize {
        self.find_and_compare_ext(template, NODE_DEFAULT)
    }

    // TODO: empty body
    //
    // XXX: last refactor levels_num
    // XXX: open template every iteration?
    // current_index
    pub fn process_template(
        &self,
        text_tree_elements: &TextTreeElements,
        level: usize,
        index: usize,
        size: usize,
        prefixes: &str,
        bitflag: usize,
    ) -> Option<Vec<String>> {
        let mut v = Vec::new();

        // get brahch text, node string and store these
        let (prefix, branch) = text_tree_elements.get_prefix_branch(level, index, size);
        let entry_str = self.to_colored_string(bitflag);
        //let entry_str = self.to_string_ext(NODE_NAME | NODE_SIZE | NODE_DESC);
        v.push(format!("{}{}{}\n", prefixes, branch, entry_str));

        // update prefixes, append current prefix
        let prefixes = format!("{}{}", prefixes, prefix);

        // process children
        if let Some(children) = &self.borrow().children {
            let size = children.len();
            for (i, child) in children.iter().enumerate() {
                if let Some(mut c) = child.process_template(
                    text_tree_elements,
                    level + 1,
                    i,
                    size,
                    &prefixes,
                    bitflag,
                ) {
                    v.append(&mut c);
                }
            }
        }

        Some(v)
    }

    // TODO: empty body
    //
    // XXX: last refactor levels_num
    // XXX: open template every iteration?
    // current_index
    //pub fn process_template(
    //&self,
    //text_tree_elements: &TextTreeElements,
    //level: usize,
    //index: usize,
    //size: usize,
    //prefixes: &str,
    //origin: &Node,
    //) -> Option<Vec<String>> {
    //let mut v = Vec::new();

    //// get brahch text
    //let (prefix, branch) = text_tree_elements.get_prefix_branch(level, index, size);
    //let changes = origin.find_and_compare_ext(&self, NODE_NAME | NODE_SIZE);
    //let color_scheme = ColorScheme::default();
    //let entry_str = self.to_colored_string_ext(&color_scheme, NODE_NAME | NODE_SIZE, changes);
    //v.push(format!("{}{}{}\n", prefixes, branch, entry_str));

    //let prefixes = format!("{}{}", prefixes, prefix);

    //// process children
    //if let Some(children) = &self.borrow().children {
    //let size = children.len();
    //for (i, child) in children.iter().enumerate() {
    //if let Some(mut c) = child.process_template(
    //text_tree_elements,
    //level + 1,
    //i,
    //size,
    //&prefixes,
    //origin,
    //) {
    //v.append(&mut c);
    //}
    //}
    //}

    //Some(v)
    //}

    /// set parent field
    pub fn set_parent(&self, parent: NodeWeak) {
        self.borrow_mut().parent = Some(parent);
    }

    // XXX: vec with keywords and output like tmux
    /// convert to string user giving fields
    pub fn to_string_ext(&self, bitflag: usize) -> String {
        let mut v = Vec::new();

        let node = self.borrow();

        if (bitflag & NODE_NAME) > 0 {
            v.push(self.borrow().file.clone().into_string().unwrap());
        }

        if (bitflag & NODE_DESC) > 0 {
            if let Some(desc) = &node.desc {
                v.push(format!("\"{}\"", desc));
            }
        }

        if (bitflag & NODE_SHA256) > 0 {
            if let Some(sha256) = &node.sha256 {
                v.push(sha256.to_string());
            }
        }

        if (bitflag & NODE_STATUS) > 0 {
            if let Some(status) = &node.status {
                v.push(status.to_string());
            }
        }

        if (bitflag & NODE_MODIFIED) > 0 {
            if let Some(modified) = &node.modified {
                v.push(modified.to_string());
            }
        }

        if (bitflag & NODE_ACCESSED) > 0 {
            if let Some(accessed) = &node.accessed {
                v.push(accessed.to_string());
            }
        }

        if (bitflag & NODE_CREATED) > 0 {
            if let Some(created) = &node.created {
                v.push(created.to_string());
            }
        }

        if (bitflag & NODE_SIZE) > 0 {
            if let Some(size) = &node.size {
                v.push(size.to_string());
            }
        }

        if (bitflag & NODE_FILE_TYPE) > 0 {
            if let Some(file_type) = &node.file_type {
                v.push(file_type.to_string());
            }
        }

        // TODO: print
        if (bitflag & NODE_TAGS) > 0 {
            unimplemented!();
            //if let Some(tags) = &node.tags {
            //v.push(tags.to_string());
            //}
        }

        v.join(" ")
    }

    //
    //pub fn for_fields(f: FnOnce)
    //{
    //if (bitflag & NODE_DESC) > 0 {
    //}
    //}

    //// XXX: if any changes yellow, removed red, new blue
    //pub fn to_colored_string(&self, fields = default) -> String {

    // same as to_colored_string_ext version, but uses node's status field as changes bitflags
    // XXX: rename
    pub fn to_colored_string(&self, fields_bitflag: usize) -> String {
        let status = self.borrow().status.unwrap_or(NODE_NOT_EXISTS);
        //println!("status: {}", &status);
        let color_scheme = ColorScheme::default();
        self.to_colored_string_ext(&color_scheme, fields_bitflag, status)
    }

    //pub fn from_color_scheme(color_scheme: &ColorScheme, bitflag: usize) -> TerminalColor {
    //if changes_bitflag == NODE_UNTRACKED {
    //} else changes_bitflag ==  {
    //}
    //}

    /// convert selected fields of node to colored string, using changes bitflags
    // TODO: colorscheme
    // NOTE: mb. if nodes compared color must be set, if untached default color, not set
    pub fn to_colored_string_ext(
        &self,
        color_scheme: &ColorScheme,
        fields_bitflag: usize,
        changes_bitflag: usize,
    ) -> String {
        let mut v = Vec::new();

        let node = self.borrow();

        if (fields_bitflag & NODE_NAME) > 0 {
            // XXX: get node name fn
            let mut name = node.clone().file.into_string().unwrap();
            // XXX: mb local var for color all elements
            if (changes_bitflag & NODE_UNTRACKED) > 0 {
                name = TerminalColor::colorize(&name, &color_scheme.untracked);
            } else if changes_bitflag == NODE_NOT_EXISTS {
                name = TerminalColor::colorize(&name, &color_scheme.removed);
            } else if changes_bitflag != NODE_NONE {
                name = TerminalColor::colorize(&name, &color_scheme.changed);
            } else if changes_bitflag == NODE_NONE {
                name = TerminalColor::colorize(&name, &color_scheme.standard);
            }
            v.push(name);
        }

        //if (fields_bitflag & NODE_NAME) > 0 {
        //// XXX: get node name fn
        //let mut name = node.clone().file.into_string().unwrap();
        //if (changes_bitflag & NODE_NAME) > 0 {
        //name = TerminalColor::colorize(&name, &color);
        //}
        //v.push(name);
        //}

        if (fields_bitflag & NODE_DESC) > 0 {
            if let Some(desc) = &node.desc {
                let mut desc = String::from(desc);
                if (changes_bitflag & NODE_DESC) > 0 {
                    desc = TerminalColor::colorize(&desc, &color_scheme.changed);
                }
                v.push(desc);
            }
        }

        if (fields_bitflag & NODE_SHA256) > 0 {
            if let Some(sha256) = &node.sha256 {
                let mut sha256 = String::from(sha256);
                if (changes_bitflag & NODE_SHA256) > 0 {
                    sha256 = TerminalColor::colorize(&sha256, &color_scheme.changed);
                }
                v.push(sha256);
            }
        }

        if (fields_bitflag & NODE_MODIFIED) > 0 {
            if let Some(modified) = &node.modified {
                let mut modified = modified.to_string();
                if (changes_bitflag & NODE_MODIFIED) > 0 {
                    modified = TerminalColor::colorize(&modified, &color_scheme.changed);
                }
                v.push(modified);
            }
        }

        if (fields_bitflag & NODE_ACCESSED) > 0 {
            if let Some(accessed) = &node.accessed {
                let mut accessed = accessed.to_string();
                if (changes_bitflag & NODE_ACCESSED) > 0 {
                    accessed = TerminalColor::colorize(&accessed, &color_scheme.changed);
                }
                v.push(accessed);
            }
        }

        if (fields_bitflag & NODE_CREATED) > 0 {
            if let Some(created) = &node.created {
                let mut created = created.to_string();
                if (changes_bitflag & NODE_CREATED) > 0 {
                    created = TerminalColor::colorize(&created, &color_scheme.changed);
                }
                v.push(created);
            }
        }

        if (fields_bitflag & NODE_SIZE) > 0 {
            if let Some(size) = &node.size {
                let mut size = size.to_string();
                if (changes_bitflag & NODE_SIZE) > 0 {
                    size = TerminalColor::colorize(&size, &color_scheme.changed);
                }
                v.push(size);
            }
        }

        if (fields_bitflag & NODE_FILE_TYPE) > 0 {
            if let Some(file_type) = &node.file_type {
                let mut file_type = file_type.to_string();
                if (changes_bitflag & NODE_FILE_TYPE) > 0 {
                    file_type = TerminalColor::colorize(&file_type, &color_scheme.changed);
                }
                v.push(file_type);
            }
        }

        //if (fields_bitflag & NODE_TAGS) > 0 {
        //if let Some(tags) = &node.tags {
        //let mut tags = tags.to_string();
        //if (changes_bitflag & NODE_TAGS) > 0 {
        //tags = TerminalColor::colorize(&tags, &color);
        //}
        //v.push(tags);
        //}
        //}

        if (fields_bitflag & NODE_COMMENT) > 0 {
            if let Some(comment) = &node.comment {
                let mut comment = comment.to_string();
                if (changes_bitflag & NODE_COMMENT) > 0 {
                    comment = TerminalColor::colorize(&comment, &color_scheme.changed);
                }
                v.push(comment);
            }
        }

        v.join(" ")
    }

    // XXX: rename copy?
    // XXX: tags
    // update all fields except desc and children
    // XXX: why compare if overwrite anyway?
    pub fn update(&self, origin: &Node) -> usize {
        let mut bitflag = NODE_NONE;
        let mut node = self.borrow_mut();
        let origin = origin.borrow();
        // XXX: not update description?
        if node.desc != origin.desc {
            node.desc = origin.desc.clone();
            bitflag |= NODE_DESC;
        }
        if node.sha256 != origin.sha256 {
            node.sha256 = origin.sha256.clone();
            bitflag |= NODE_SHA256;
        }
        if node.modified != origin.modified {
            node.modified = origin.modified.clone();
            bitflag |= NODE_MODIFIED;
        }
        if node.accessed != origin.accessed {
            node.accessed = origin.accessed.clone();
            bitflag |= NODE_ACCESSED;
        }
        if node.created != origin.created {
            node.created = origin.created.clone();
            bitflag |= NODE_CREATED;
        }
        if node.size != origin.size {
            node.size = origin.size.clone();
            bitflag |= NODE_SIZE;
        }
        if node.file_type != origin.file_type {
            node.file_type = origin.file_type.clone();
            bitflag |= NODE_FILE_TYPE;
        }
        if node.tags != origin.tags {
            node.tags = origin.tags.clone();
            bitflag |= NODE_TAGS;
        }
        bitflag
    }

    // XXX: rename copy or smthg
    /// CRUD: update
    /// update user given fields of the current node
    pub fn update_ext(&mut self, origin: &NodeData, bitflag: usize) {
        let mut node = self.borrow_mut();
        //if (bitflag & COMPARE_NAME) > 0 {
        //name = &origin_name.clone();
        //}
        if (bitflag & NODE_DESC) > 0 {
            node.desc = origin.desc.clone();
        }
        if (bitflag & NODE_SHA256) > 0 {
            node.sha256 = origin.sha256.clone();
        }
        if (bitflag & NODE_STATUS) > 0 {
            node.status = origin.status;
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            node.modified = origin.modified.clone();
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            node.accessed = origin.accessed.clone();
        }
        if (bitflag & NODE_CREATED) > 0 {
            node.created = origin.created.clone();
        }
        if (bitflag & NODE_SIZE) > 0 {
            node.size = origin.size;
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            node.file_type = origin.file_type.clone();
        }
        if (bitflag & NODE_TAGS) > 0 {
            node.tags = origin.tags.clone();
        }
        if (bitflag & NODE_COMMENT) > 0 {
            node.comment = origin.comment.clone();
        }
    }

    // XXX: optimize, return error
    pub fn from_tree_entry(parent: Option<NodeWeak>, entry: &TreeEntry) -> Self {
        let data = NodeData::default();
        let node = Node(Rc::new(RefCell::new(data)));

        // if entry contains body and name
        if let Some((name, Some(body))) = entry.first() {
            // convert children first
            let node_children = if let Some(children) = &body.children {
                let mut nodes = Vec::new();
                for child in children {
                    let parent = Rc::downgrade(&node.0);
                    nodes.push(Node::from_tree_entry(Some(parent), child));
                }
                Some(nodes)
            } else {
                None
            };

            let mut data = node.borrow_mut();
            // clone data
            data.file = OsString::from(name);
            data.parent = parent;
            data.desc = body.desc.clone();
            data.children = node_children;
            data.tags = body.tags.clone();
            data.sha256 = body.sha256.clone();
            data.status = body.status;
            data.modified = body.modified.clone();
            data.accessed = body.accessed.clone();
            data.created = body.created.clone();
            data.size = body.size;
            data.file_type = body.file_type.clone();
            data.compare = body.compare;
        }

        node
    }

    pub fn load<P: AsRef<Path>>(file: P) -> Result<Node, ()> {
        if let Ok(tree) = FileTree::read(file) {
            if let Some(entry) = tree.tree {
                Ok(Node::from_tree_entry(None, &entry))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn save<P: AsRef<Path>>(self, file: P) -> Result<(), Error> {
        let entry = TreeEntry::from(&self);
        let tree = FileTree::new(Some(entry), None);
        tree.write(file)
    }
}

//impl From<TreeEntry> for Node {
//fn from(tree_entry: TreeEntry) -> Self {
//if let Some((name, body)) = tree_entry.first() {
//let mut node = Node::new(OsString::from(name.to_owned()), None);
//if let Some(body) = body {
////node.desc = body.desc.to_owned();
//if let Some(children) = &body.children {
//for child in children {
//let mut child_node = Node::from(child.to_owned());
//child_node.parent = Some(Rc::new(RefCell::new(node.to_owned())));
//node.add_child(child_node);
//}
//}
//}
//return node;
//// empty return
//} else {
//let node = Node::new(OsString::new(), None);
//return node;
//}
//}
//}

// TODO: From<Path> to Type dir, symlink, file
// create from DirEntry
impl From<&DirEntry> for Node {
    fn from(dir_entry: &DirEntry) -> Self {
        Node(Node::from_dir_entry_ext(None, dir_entry, NODE_DEFAULT))
    }
}

// NOTE: impossible, parent arg is missing
//impl From<&TreeEntry> for Node {
//fn from(entry: &TreeEntry) -> Self {

//pub struct NodeIterator {
//pub previous: Option<Node>,
//pub next: Option<Node>,
//}

//impl Iterator for NodeIterator {
//type Item = Node;

//fn next(&mut self) -> Option<Node> {
//self.previous.to_owned()
//}
//}
