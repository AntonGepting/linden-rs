#[test]
fn path() {
    use std::path::Path;

    let p = Path::new(".");
    let cs = p.components();
    dbg!(cs);
    let p = Path::new("/**/*.rs");
    let cs = p.components();
    dbg!(cs);
    let p = Path::new("/**/?abc.rs");
    let cs = p.components();
    dbg!(cs);
}

#[test]
fn new() {
    use crate::file_tree::Node;
    use std::ffi::OsString;

    let node = Node::new(OsString::from("."), None);
    assert_eq!(node.borrow().file, ".");

    let node = Node::new(".", None);
    assert_eq!(node.borrow().file, ".");

    let node = Node::new(".".to_string(), None);
    assert_eq!(node.borrow().file, ".");
}

#[test]
fn add_child() {
    use crate::file_tree::Node;
    use std::ffi::OsString;

    let root = Node::to_node(Node::new(".", None));
    let child1 = Node::new("a", None);
    let child2 = Node::new("b", None);
    let child3 = Node::new("c", None);

    root.add_child(child1);

    //dbg!(root);
}

#[test]
fn remove_child() {
    use crate::file_tree::Node;
    use std::ffi::OsString;

    let root = Node::to_node(Node::new(".", None));
    let child1 = Node::new("a", None);
    let child2 = Node::new("b", None);
    let child3 = Node::new("c", None);

    root.add_child(child1);
    root.add_child(child2);
    root.add_child(child3);

    root.remove_child("c");
    dbg!(root);
}

#[test]
fn remove() {
    use crate::file_tree::Node;
    use std::ffi::OsString;

    let root = Node::to_node(Node::new(".", None));
    let child1 = Node::new("a", None);
    let child2 = Node::new("b", None);
    let child3 = Node::new("c", None);

    root.add_child(child1);
    root.add_child(child2);
    root.add_child(child3);

    root.remove("./c");
    dbg!(root);
}

#[test]
fn add_path_ext() {
    use crate::file_tree::{Node, NODE_DEFAULT};
    use std::path::Path;

    let mut node = Node::default();
    let ignore = vec![".git".to_string(), "target".to_string()];

    node.add_path_ext(None, Path::new("."), Some(&ignore), NODE_DEFAULT)
        .unwrap();
    assert_eq!(node.borrow().file, ".");

    node.add_path_ext(None, &Path::new("."), Some(&ignore), NODE_DEFAULT)
        .unwrap();
    assert_eq!(node.borrow().file, ".");

    node.add_path_ext(None, ".".to_string(), Some(&ignore), NODE_DEFAULT)
        .unwrap();
    assert_eq!(node.borrow().file, ".");

    node.add_path_ext(None, ".", Some(&ignore), NODE_DEFAULT)
        .unwrap();
    assert_eq!(node.borrow().file, ".");
}

#[test]
fn from_tree_entry() {
    use crate::file_tree::{Node, TreeEntry, TreeEntryBody};

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

#[test]
fn get() {
    use crate::file_tree::Node;
    use std::path::Path;

    let root = Node::to_node(Node::new(".", None));
    let child1 = Node::new("a", None);
    let child2 = Node::new("b", None);
    let child3 = Node::new("c", None);

    root.add_child(child1);
    root.add_child(child2);
    root.add_child(child3);

    dbg!(root.get_child("a"));
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
