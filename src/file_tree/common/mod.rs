pub mod error;
pub mod file_type;
//pub use file_type::FileType;

// XXX: alphabetical?
// info from filesystem
pub const NODE_NAME: usize = 1;
pub const NODE_SHA256: usize = 1 << 1;
pub const NODE_STATUS: usize = 1 << 2;
pub const NODE_MODIFIED: usize = 1 << 3;
pub const NODE_ACCESSED: usize = 1 << 4;
pub const NODE_CREATED: usize = 1 << 5;
pub const NODE_SIZE: usize = 1 << 6;
pub const NODE_FILE_TYPE: usize = 1 << 7;
pub const NODE_CHILDREN: usize = 1 << 8;
//pub const NODE_COMPARE: usize = 1 << 9;
pub const NODE_FULL_PATH: usize = 1 << 10;

// info from user
pub const NODE_DESC: usize = 1 << 11;
pub const NODE_TAGS: usize = 1 << 12;
pub const NODE_COMMENT: usize = 1 << 13;
pub const NODE_HIDDEN: usize = 1 << 14;

// other info
pub const NODE_NOT_EXISTS: usize = 1 << 15;
//pub const NODE_NEW: usize = 1 << 16;
pub const NODE_UNTRACKED: usize = 1 << 16;
//pub const NODE_REMOVED: usize = 1 << 16;
//pub const NODE_CHANGED: usize = 1 << 16;

pub const SORT_ASC: usize = 1 << 23;
pub const SORT_DSC: usize = 1 << 24;

/// automatic fields (auto generated or specified by os or file functions)
pub const NODE_AUTO: usize = NODE_NAME
    | NODE_SHA256
    | NODE_STATUS
    | NODE_MODIFIED
    | NODE_ACCESSED
    | NODE_CREATED
    | NODE_SIZE
    | NODE_FILE_TYPE
    | NODE_CHILDREN
    //| NODE_COMPARE
    | NODE_FULL_PATH;
/// manual fields (given by user)
pub const NODE_MANUAL: usize = NODE_DESC | NODE_TAGS | NODE_COMMENT | NODE_HIDDEN;

pub const NODE_NONE: usize = 0;
pub const NODE_DEFAULT: usize = NODE_NAME | NODE_DESC | NODE_SIZE | NODE_FILE_TYPE | NODE_COMMENT;
pub const NODE_ALL: usize = NODE_NAME
    | NODE_DESC
    | NODE_SHA256
    | NODE_STATUS
    | NODE_MODIFIED
    | NODE_ACCESSED
    | NODE_CREATED
    | NODE_SIZE
    | NODE_FILE_TYPE
    | NODE_CHILDREN
    //| NODE_COMPARE
    | NODE_TAGS
    | NODE_FULL_PATH
    | NODE_COMMENT;

// fields option except children
pub const NODE_OPTIONALS: usize = NODE_DESC
    | NODE_SHA256
    | NODE_STATUS
    | NODE_MODIFIED
    | NODE_ACCESSED
    | NODE_CREATED
    | NODE_SIZE
    | NODE_FILE_TYPE
    //| NODE_COMPARE
    | NODE_TAGS;

pub const NODE_BASICS: usize = NODE_NAME | NODE_SIZE;

pub const NODE_DYNAMICS: usize =
    NODE_SHA256 | NODE_MODIFIED | NODE_ACCESSED | NODE_CREATED | NODE_SIZE;

////pub mod file_tree;
////pub mod file_tree_tests;

////pub mod tree_entries;
////pub mod tree_entries_tests;
//pub mod tree_entry;
//pub mod tree_entry_tests;

//pub mod tree_entry_body;

////pub use self::file_tree::FileTree;
////pub use tree_entries::TreeEntries;
//pub use tree_entry::TreeEntry;
//pub use tree_entry_body::TreeEntryBody;
