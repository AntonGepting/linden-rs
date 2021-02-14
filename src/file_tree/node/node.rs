//#![forbid(unsafe_code)]
//#![warn(missing_docs)]

use crate::file_tree::{Error, FileTree, FileType, TreeEntry};
use crate::file_tree::{
    NODE_ACCESSED, NODE_ALL, NODE_CHILDREN, NODE_CREATED, NODE_DEFAULT, NODE_DESC, NODE_FILE_TYPE,
    NODE_MODIFIED, NODE_NAME, NODE_NONE, NODE_SHA256, NODE_SIZE, NODE_STATUS, NODE_TAGS, SORT_ASC,
};
use chrono::offset::Utc;
use chrono::DateTime;
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::DirEntry;
use std::path::{Component, Path, PathBuf};
use std::rc::{Rc, Weak};

/// default/child pointer
type NodeRc = Rc<RefCell<NodeData>>;
/// parent pointer
type NodeWeak = Weak<RefCell<NodeData>>;

/// newtype wrapper
#[derive(Debug, Clone, Default)]
pub struct Node(NodeRc);

/// main structure holding data
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
    /// ?
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

impl Node {
    // add child, create children vector if not exist (for first child)
    pub fn add_child(&self, child: NodeRc) {
        let node = Node(child);
        if self.has_children() {
            if let Some(ref mut children) = self.borrow_mut().children {
                children.push(node)
            }
        } else {
            self.borrow_mut().children = Some(vec![node])
        }
    }

    //pub fn borrow(&self) -> Ref<Node> {
    //self.0.borrow()
    //}

    //pub fn len(&self) -> usize {
    //match &self.0.borrow().children {
    //Some(children) => children.len(),
    //None => 0,
    //}
    //}

    // add files and directories to self (TreeEntry)
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
            let name = dir_entry.file_name().into_string().unwrap();
            if let Some(ref ignore_vec) = ignore {
                if ignore_vec.contains(&name) {
                    continue;
                }
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

    // XXX: string as output
    pub fn bitflag_to_string(bitflag: usize) -> String {
        let mut s = String::new();
        if (bitflag & NODE_DESC) > 0 {
            s = format!("{} desc ", s);
        }
        if (bitflag & NODE_SHA256) > 0 {
            s = format!("{} sha256 ", s);
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            s = format!("{} modified ", s);
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            s = format!("{} accessed ", s);
        }
        if (bitflag & NODE_CREATED) > 0 {
            s = format!("{} created ", s);
        }
        if (bitflag & NODE_SIZE) > 0 {
            s = format!("{} size ", s);
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            s = format!("{} type ", s);
        }
        if (bitflag & NODE_TAGS) > 0 {
            s = format!("{} tags ", s);
        }
        s
    }

    // clear given fields
    pub fn clear_ext(&self, bitflag: usize) {
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

        // for children too
        if (bitflag & NODE_CHILDREN) > 0 {
            if let Some(children) = &mut self.borrow_mut().children {
                for child in children {
                    child.clear_ext(bitflag);
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
    pub fn compare(&self, origin: &Node) -> bool {
        let node = self.borrow();
        let origin = origin.borrow();

        let compare = node.compare.unwrap_or(NODE_DEFAULT);

        if (compare & NODE_NAME) == NODE_NAME {
            if node.file == origin.file {
                return true;
            }
        }
        if (compare & NODE_SHA256) == NODE_SHA256 {
            if node.sha256 == origin.sha256 {
                return true;
            }
        }
        false
    }

    // XXX: time older younger?
    // XXX: tags
    //
    // returns what fields are different
    pub fn compare_ext(&self, origin: &Node) -> usize {
        let mut bitflag = NODE_NONE;

        let node = self.borrow();
        let origin = origin.borrow();

        if node.file != origin.file {
            bitflag |= NODE_NAME;
        }
        if node.desc != origin.desc {
            bitflag |= NODE_DESC;
        }
        if node.sha256 != origin.sha256 {
            bitflag |= NODE_SHA256;
        }
        if node.modified != origin.modified {
            bitflag |= NODE_MODIFIED;
        }
        if node.accessed != origin.accessed {
            bitflag |= NODE_ACCESSED;
        }
        if node.created != origin.created {
            bitflag |= NODE_CREATED;
        }
        if node.size != origin.size {
            bitflag |= NODE_SIZE;
        }
        if node.file_type != origin.file_type {
            bitflag |= NODE_FILE_TYPE;
        }
        if node.tags != origin.tags {
            bitflag |= NODE_TAGS;
        }
        bitflag
    }

    // XXX: -1?
    pub fn children_num(&self) -> usize {
        match &self.0.borrow().children {
            Some(children) => children.len(),
            None => 0,
        }
    }

    // add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: overwrites pure file name of the root directory with full path
    pub fn create_from_path(dir: &Path, ignore: Option<&Vec<String>>) -> Result<Node, Error> {
        Node::create_from_path_ext(dir, ignore, NODE_DEFAULT)
    }

    // add files and directories to self (TreeEntry)
    // XXX: additional options as an argument
    // NOTE: overwrites pure file name of the root directory with full path
    pub fn create_from_path_ext(
        dir: &Path,
        ignore: Option<&Vec<String>>,
        bitflag: usize,
    ) -> Result<Node, Error> {
        // if path is convertable into str
        let file = dir.as_os_str();
        let mut node = Node(Node::new(file.to_owned(), None));

        node.add_path_ext(None, dir, ignore, bitflag)?;
        Ok(node)
    }

    pub fn desc(&self, desc: Option<String>) {
        self.borrow_mut().desc = desc;
    }

    pub fn comment(&self, comment: Option<String>) {
        self.borrow_mut().comment = comment;
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

            //permissions
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
    pub fn get_child(&self, file: &OsStr) -> Option<Node> {
        // OPT: self.children.and_then(|children| children.iter)
        if let Some(children) = &self.borrow().children {
            for child in children {
                if child.borrow().file == *file {
                    return Some(child.clone());
                }
            }
        }
        None
    }

    pub fn borrow(&self) -> Ref<NodeData> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<NodeData> {
        self.0.borrow_mut()
    }

    // get full path and file name of entry
    pub fn get_full_path(this: &NodeData) -> PathBuf {
        let mut path = Node::get_path(&this);
        path.push(&this.file);
        path
    }

    pub fn get_parent(&self) -> Option<NodeRc> {
        match &self.0.borrow().parent {
            Some(parent) => parent.upgrade(),
            None => None,
        }
    }

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

    // get full path using parents fields excluding file name for given entry
    //
    // XXX: follow symlinks?
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

    pub fn get_tabs(
        level: usize,
        current: usize,
        len: usize,
        tab_template: [&str; 3],
        link_template: [&str; 3],
    ) -> (String, String) {
        let (tab, link) = if level == 0 {
            // first -> no tab, no branch
            (tab_template[0], link_template[0])
        } else if current == len - 1 {
            // last -> tab + single last branch
            (tab_template[2], link_template[2])
        } else {
            // regular -> parent link + double branch (current & next item)
            (tab_template[1], link_template[1])
        };
        (tab.to_string(), link.to_string())
    }

    //pub fn remove_child(&self)

    pub fn has_children(&self) -> bool {
        self.borrow().children.is_some()
    }

    pub fn ls(&self, path: &Path) -> Result<(), Error> {
        let s = self.to_string_ext(NODE_DEFAULT);
        println!("{}", s);
        self.for_children(path, &|child, _i, _size| {
            //let s = child.to_string_ext(COMPARE_DEFAULT);
            //println!("{}", s);
            child.ls(path).unwrap_or(());
        })
    }

    pub fn new(file: OsString, parent: Option<NodeWeak>) -> NodeRc {
        Rc::new(RefCell::new(NodeData {
            file,
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
            let (tab, link) =
                Node::get_tabs(level, idx, size, ["", "│  ", "   "], ["", "├─ ", "└─ "]);
            println!("{}{}{}", tab, link, s);
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

    // TODO: empty body
    //
    // XXX: last refactor levels_num
    // XXX: open template every iteration?
    // current_index
    pub fn process_template(
        &self,
        level: usize,
        idx: usize,
        size: usize,
        tabs: &str,
        template: &Path,
    ) -> Option<String> {
        //use tera::Context;
        //use tera::Tera;

        //let mut tera = Tera::default();
        //let mut tera = Tera::new(template.to_str().unwrap()).unwrap();
        //tera.add_template_file(template, None).unwrap();

        //let mut context = Context::new();
        //context.insert("entry", &self);

        // render entry
        //let entry_str = tera.render(&template.to_str().unwrap(), &context).unwrap();
        let entry_str = self.to_string_ext(NODE_NAME | NODE_DESC | NODE_TAGS);
        let (tab, link) = Node::get_tabs(level, idx, size, ["", "│  ", "   "], ["", "├─ ", "└─ "]);
        let mut render = format!("{}{}\n", link, entry_str);
        let tabs = format!("{}{}", tabs, tab);

        // render children
        if let Some(children) = &self.borrow().children {
            // calculating tabs

            //entry_str = format!("├{}", render);
            let mut children_acc = "".to_string();
            for (i, child) in children.iter().enumerate() {
                if let Some(child_str) =
                    child.process_template(level + 1, i, children.len(), &tabs, template)
                {
                    //render = format!("{}{}{}", render, tab, child_str);
                    children_acc = format!("{}{}{}", children_acc, tabs, child_str);
                }
            }
            render = format!("{}{}", render, children_acc);
        }
        //println!("{}", render);
        Some(render)
    }

    // TODO: write it
    // XXX: Option
    pub fn remove(&self, _path: &Path) -> Option<()> {
        // XXX: examples check is it properly
        //path.file_name().and_then(|last_element| {
        //last_element.to_str().and_then(|_name| {
        //self.get_parent(&path).as_mut().and_then(|parent| {
        //let mut parent = parent.borrow_mut();
        //if let Some(body) = parent.first_body_mut() {
        //body.children = None;
        //}
        Some(())
        //Some(parent.remove_child(name))
        //})
        //})
        //})
    }

    pub fn set_parent(&self, parent: NodeWeak) {
        self.borrow_mut().parent = Some(parent);
    }

    pub fn to_string_ext(&self, bitflag: usize) -> String {
        let mut s = "".to_string();
        if (bitflag & NODE_NAME) > 0 {
            //s = format!("{}{:50} ", s, name);
            s = format!(
                "{}{} ",
                s,
                &self.borrow().file.clone().into_string().unwrap()
            );
        }
        if (bitflag & NODE_DESC) > 0 {
            if let Some(desc) = &self.borrow().desc {
                s = format!("{}\"{}\" ", s, desc);
            }
        }
        if (bitflag & NODE_SHA256) > 0 {
            if let Some(sha256) = &self.borrow().sha256 {
                s = format!("{}{} ", s, sha256);
            }
        }
        if (bitflag & NODE_STATUS) > 0 {
            if let Some(status) = &self.borrow().status {
                s = format!("{}{} ", s, status);
            }
        }
        if (bitflag & NODE_MODIFIED) > 0 {
            if let Some(modified) = &self.borrow().modified {
                s = format!("{}{} ", s, modified);
            }
        }
        if (bitflag & NODE_ACCESSED) > 0 {
            if let Some(accessed) = &self.borrow().accessed {
                s = format!("{}{} ", s, accessed);
            }
        }
        if (bitflag & NODE_CREATED) > 0 {
            if let Some(created) = &self.borrow().created {
                s = format!("{}{} ", s, created);
            }
        }
        if (bitflag & NODE_SIZE) > 0 {
            if let Some(size) = &self.borrow().size {
                s = format!("{}{} ", s, size);
            }
        }
        if (bitflag & NODE_FILE_TYPE) > 0 {
            if let Some(file_type) = &self.borrow().file_type {
                s = format!("{}{} ", s, file_type);
            }
        }
        // TODO: print
        if (bitflag & NODE_TAGS) > 0 {
            if let Some(tags) = &self.borrow().tags {
                s = format!("{} {:?}", s, tags);
            }
        }
        s
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
    pub fn update_ext(&mut self, origin: &Node, bitflag: usize) {
        let mut node = self.borrow_mut();
        let origin = origin.borrow();
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
