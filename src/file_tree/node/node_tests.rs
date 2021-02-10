#[test]
fn path() {
    use std::path::Path;

    let p = Path::new(".");
    let cs = p.components();
    dbg!(cs);
}

#[test]
fn new() {
    use crate::node::node::Node;
    use std::ffi::OsString;

    let node = Node::new(OsString::from("."), None);
    dbg!(node);
}

#[test]
fn add_child() {
    use crate::node::node::Node;
    use std::ffi::OsString;
    use std::rc::Rc;

    let x = Node::new(OsString::from("."), None);
    let y = Node::new(OsString::from("a"), Some(Rc::downgrade(&x)));
    let x = Node::to_node(x);
    x.add_child(y);

    dbg!(x);
}

#[test]
fn add_path_ext() {
    use crate::{Node, COMPARE_DEFAULT};
    use std::path::Path;

    let mut node = Node::default();
    let ignore = vec![".git".to_string(), "target".to_string()];
    node.add_path_ext(None, &Path::new("."), Some(&ignore), COMPARE_DEFAULT)
        .unwrap();

    dbg!(node);
    //node.print_in_line();
}

#[test]
fn from_tree_entry() {
    use crate::{Node, TreeEntry, TreeEntryBody};

    let child11 = TreeEntry::new("child11", TreeEntryBody::default());
    let child12 = TreeEntry::new("child12", TreeEntryBody::default());
    let child13 = TreeEntry::new("child13", TreeEntryBody::default());
    let child1_body = TreeEntryBody {
        children: Some(vec![child11, child12, child13]),
        ..Default::default()
    };
    let child1 = TreeEntry::new("child1", child1_body);
    let child2 = TreeEntry::new("child2", TreeEntryBody::default());
    let child3 = TreeEntry::new("child3", TreeEntryBody::default());
    let root_body = TreeEntryBody {
        children: Some(vec![child1, child2, child3]),
        ..Default::default()
    };
    let root = TreeEntry::new("root", root_body);

    let node = Node::from_tree_entry(None, &root);

    dbg!(node);
}

//#[test]
//fn print_in_line() {
//use crate::node::node::Node;
//use crate::tree_entry::tree_entry::COMPARE_DEFAULT;
//use std::path::Path;

//let ignore = vec![".git".to_string(), "target".to_string()];

//let mut node = Node::default();
//node.add_path_ext(&Path::new("."), Some(&ignore), COMPARE_DEFAULT);
//node.print_in_line();
//}

//#[test]
//fn create_from_path() {
//use crate::node::node::Node;
//use std::path::Path;

//let ignore = vec![".git".to_string(), "target".to_string()];
//let node = Node::create_from_path(Path::new("."), Some(&ignore)).unwrap();
//}

//#[test]
//fn aadf() {
//use crate::node::node::Node;
//use crate::tree_entry::TreeEntry;
//use std::path::Path;

//let ignore = vec![".git".to_string(), "target".to_string()];
//let node = Node::create_from_path(Path::new("."), Some(&ignore)).unwrap();

//let tree_entry = TreeEntry::from(&node);
//dbg!(tree_entry);
//}
