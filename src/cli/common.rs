use super::cli::Cli;
use crate::file_tree::{
    NODE_ACCESSED, NODE_CHILDREN, NODE_CREATED, NODE_DESC, NODE_FILE_TYPE, NODE_MODIFIED,
    NODE_NAME, NODE_NONE, NODE_SIZE, NODE_TAGS,
};
use clap::ArgMatches;

pub const LOG_CLI_NO_SUBCOMMAND_RECIEVED: &str = "no subcommand recognized";
pub const APP_NAME: &str = "Linden";
//const DEFAULT_PROJECT_FILENAME: &'static str = "project.mosaic.yml";

pub const APP_VERSION: &str = "0.0.1";
pub const APP_ABOUT: &str = "build file tree, render output";
pub const APP_AUTHOR: &str = "Anton Gepting <anton.gepting@gmail.com>";

pub const LOG_CLI_END: &str = "exit";
//const CMD_NEW: &'static str = "new";
//const CMD_NEW_TEXT: &'static str = "Create new project";

/// `ls [-b <DB>] [PATH] [-a, --a <FLAGS>]`
///   - `-b <DB>, --database=<DB>` - specify db file (default: `.tree.yml`)
///   - `[PATH]` - (default: `.`)
pub const CMD_LS: &str = "ls";
pub const CMD_LS_TEXT: &str = "list tracked files";

/// `init [-b <DB>] [-f, --force]
/// [-i, --ignore <IGNORE_FILE1>, <IGNORE_FILE2>]` - initialize tree
///   - `-b <DB>, --database=<DB>` - specify db file (default: `.tree.yml`)
///   - `-i <IGNORE_FILE1>, --ignore=<IGNORE_FILE1>` - ignore list
///   - `-f, --force` - overwrite, if db already exist
///   - `[PATH]` - (default: `.`)
///
///  - `-A, --all`
///  - `-D, --default`
///
///  - `-N, --name`
///  - `-D, --description`
///  - `-T, --type`
///  - `-S, --size`
///  - `-C, --created`
///  - `-M, --modified`
///  - `-A, --accessed`
//  XXX: mb -R, --recourse
///  - `-H, --children`
///  - `-G, --tags`
///
pub const CMD_INIT: &str = "init";
pub const CMD_INIT_TEXT: &str = "initialize file tree";

// XXX: mb CRUD?
// edit [-b <DB>]
pub const CMD_EDIT: &str = "edit";
pub const CMD_EDIT_TEXT: &str = "edit file entry";
// read [-b <DB>]
pub const CMD_READ: &str = "read";
pub const CMD_READ_TEXT: &str = "read file entry";
// print [-b <DB>]
pub const CMD_PRINT: &str = "export";
pub const CMD_PRINT_TEXT: &str = "print tree";
// delete [-b <DB>] <FILE> [-sdamcst]
pub const CMD_DELETE: &str = "delete";
pub const CMD_DELETE_TEXT: &str = "delete file entry";
// print [-b <DB>]
pub const CMD_PRINT_META: &str = "print";
pub const CMD_PRINT_META_TEXT: &str = "print tree meta information";
// status [-b <DB>]
pub const CMD_STATUS: &str = "status";
pub const CMD_STATUS_TEXT: &str = "status information";
// XXX: sort [-b <DB>] [-z --sort-by-key, -s, --sort-order asc, desc, dir, file, symlink]
/// sort [-b <DB>] [-z --sort-by-key, -s, --sort-order asc, desc]
///  -, --name
///  -, --description
///  -, --type
///  -, --size
///  -, --created
///  -, --modified
///  -, --accessed
///
///  -, --ascending
///  -, --descending
pub const CMD_SORT: &str = "sort";
pub const CMD_SORT_TEXT: &str = "sort tree";
// merge
//pub const CMD_MERGE: &str = "merge";
//pub const CMD_MERGE_TEXT: &str = "merge tree";
// update
pub const CMD_UPDATE: &str = "update";
//pub const CMD_UPDATE_TEXT: &str = "update tree";

/// add [-b <DB>] [PATH] [-r]
/// add file/path
///  -b <DB>, --database <DB>    DB file path
///  PATH               entry file path
///
///  -r, --recourse     add children
pub const CMD_ADD: &str = "add";
pub const CMD_ADD_TEXT: &str = "add path or file";

/// clear [-b <DB>] [PATH] [-NDTSCMAHG]
/// remove field
///
///  -b <DB>, --database <DB>    DB file path
///  PATH               entry file path
///
///  -N, --name
///  -D, --description
///  -T, --type
///  -S, --size
///  -C, --created
///  -M, --modified
///  -A, --accessed
//  XXX: mb -R, --recourse
///  -H, --children
///  -G, --tags
///
pub const CMD_CLEAR: &str = "clear";
pub const CMD_CLEAR_TEXT: &str = "clear fields";

pub const KEY_BITFLAG_NAME: &str = "NAME";
pub const KEY_BITFLAG_NAME_SHORT: &str = "-N";
pub const KEY_BITFLAG_NAME_LONG: &str = "--name";
pub const KEY_BITFLAG_NAME_HELP: &str = "name";
pub const KEY_BITFLAG_DESC: &str = "DESC";
pub const KEY_BITFLAG_DESC_SHORT: &str = "-D";
pub const KEY_BITFLAG_DESC_LONG: &str = "--description";
pub const KEY_BITFLAG_DESC_HELP: &str = "description";
//pub const KEY_BITFLAG_SHA256: &str = "SHA256";
//pub const KEY_BITFLAG_SHA256_SHORT: &str = "-h";
//pub const KEY_BITFLAG_STATUS: &str = "STATUS";
//pub const KEY_BITFLAG_STATUS_SHORT: &str = "-s";
pub const KEY_BITFLAG_MODIFIED: &str = "MODIFIED";
pub const KEY_BITFLAG_MODIFIED_SHORT: &str = "-M";
pub const KEY_BITFLAG_MODIFIED_LONG: &str = "--modified";
pub const KEY_BITFLAG_MODIFIED_HELP: &str = "modified";
pub const KEY_BITFLAG_ACCESSED: &str = "ACCESSED";
pub const KEY_BITFLAG_ACCESSED_SHORT: &str = "-A";
pub const KEY_BITFLAG_ACCESSED_LONG: &str = "--accessed";
pub const KEY_BITFLAG_ACCESSED_HELP: &str = "accessed";
pub const KEY_BITFLAG_CREATED: &str = "CREATED";
pub const KEY_BITFLAG_CREATED_SHORT: &str = "-C";
pub const KEY_BITFLAG_CREATED_LONG: &str = "--created";
pub const KEY_BITFLAG_CREATED_HELP: &str = "created";
pub const KEY_BITFLAG_SIZE: &str = "SIZE";
pub const KEY_BITFLAG_SIZE_SHORT: &str = "-S";
pub const KEY_BITFLAG_SIZE_LONG: &str = "--size";
pub const KEY_BITFLAG_SIZE_HELP: &str = "size";
pub const KEY_BITFLAG_FILE_TYPE: &str = "FILE_TYPE";
pub const KEY_BITFLAG_FILE_TYPE_SHORT: &str = "-T";
pub const KEY_BITFLAG_FILE_TYPE_LONG: &str = "--type";
pub const KEY_BITFLAG_FILE_TYPE_HELP: &str = "type";
pub const KEY_BITFLAG_CHILDREN: &str = "CHILDREN";
pub const KEY_BITFLAG_CHILDREN_SHORT: &str = "-H";
pub const KEY_BITFLAG_CHILDREN_LONG: &str = "--children";
pub const KEY_BITFLAG_CHILDREN_HELP: &str = "children";
pub const KEY_BITFLAG_TAGS: &str = "TAGS";
pub const KEY_BITFLAG_TAGS_SHORT: &str = "-G";
pub const KEY_BITFLAG_TAGS_LONG: &str = "--tags";
pub const KEY_BITFLAG_TAGS_HELP: &str = "tags";

pub const KEY_BITFLAG_SORT_ORDER: &str = "ORDER_ASC";
pub const KEY_BITFLAG_SORT_ORDER_SHORT: &str = "-r";
pub const KEY_BITFLAG_SORT_ORDER_LONG: &str = "--sort-order";
pub const KEY_BITFLAG_SORT_ORDER_HELP: &str = "sort order";
pub const KEY_BITFLAG_SORT_ORDER_ASC: &str = "asc";
pub const KEY_BITFLAG_SORT_ORDER_DESC: &str = "desc";

pub const KEY_PATH: &str = "PATH";
pub const KEY_PATH_HELP: &str = "path of file";

pub const KEY_FILE_NAME: &str = "FILE";
pub const KEY_FILE_NAME_HELP: &str = "file name";

pub const KEY_RECOURSIVE: &str = "recoursive";
pub const KEY_RECOURSIVE_SHORT: &str = "-r";
pub const KEY_RECOURSIVE_LONG: &str = "--recoursive";
pub const KEY_RECOURSIVE_HELP: &str = "recoursive operation";

pub const KEY_FORCE: &str = "FORCE";
pub const KEY_FORCE_SHORT: &str = "-f";
pub const KEY_FORCE_LONG: &str = "--force";
pub const KEY_FORCE_HELP: &str = "Force overwrite if file already exists";

pub const KEY_DB: &str = "database";
pub const KEY_DB_SHORT: &str = "-b";
pub const KEY_DB_LONG: &str = "--database";
pub const KEY_DB_HELP: &str = "User defined database file";
pub const KEY_DB_FILE: &str = "DATABASE_FILE";
pub const DEFAULT_DB_FILENAME: &str = ".tree.yml";

pub const KEY_DESC: &str = "description";
pub const KEY_DESC_SHORT: &str = "-d";
pub const KEY_DESC_LONG: &str = "--description";
pub const KEY_DESC_HELP: &str = "Description";
pub const KEY_DESCRIPTION: &str = "DESCRIPTION";

// CLI keys, parameters, etc
//const KEY_PROJECT: &'static str = "project";
//const KEY_PROJECT_SHORT: &'static str = "-p";
//const KEY_PROJECT_LONG: &'static str = "--project";
//const KEY_PROJECT_HELP: &'static str = "User defined project file";
//const KEY_PROJECT_FILE: &'static str = "PROJECT_FILE";

pub const KEY_TEMPLATE: &str = "template";
pub const KEY_TEMPLATE_SHORT: &str = "-t";
pub const KEY_TEMPLATE_LONG: &str = "--template";
pub const KEY_TEMPLATE_HELP: &str = "User defined template file";
pub const KEY_TEMPLATE_FILE: &str = "TEMPLATE_FILE";
pub const DEFAULT_TEMPLATE_FILENAME: &str = "templates/entry.txt.j2";

pub const KEY_OUTPUT: &str = "output";
pub const KEY_OUTPUT_SHORT: &str = "-o";
pub const KEY_OUTPUT_LONG: &str = "--output";
pub const KEY_OUTPUT_HELP: &str = "User defined output file";
pub const KEY_OUTPUT_FILE: &str = "OUTPUT_FILE";
pub const DEFAULT_OUTPUT_FILENAME: &str = "tree.txt";

pub const KEY_LOG: &str = "log";
pub const KEY_LOG_SHORT: &str = "-l";
pub const KEY_LOG_LONG: &str = "--log";
pub const KEY_LOG_HELP: &str = "Specify log file";
pub const KEY_LOG_FILE: &str = "LOG_FILE";
pub const DEFAULT_LOG_FILENAME: &str = "linden.log";

pub const KEY_IGNORE: &str = "ignore";
pub const KEY_IGNORE_SHORT: &str = "-i";
pub const KEY_IGNORE_LONG: &str = "--ignore";
pub const KEY_IGNORE_HELP: &str = "ignore list files";
//pub const KEY_IGNORE_FILE1: &str = "IGNORE_FILE1";
//pub const KEY_IGNORE_FILE2: &str = "IGNORE_FILE2";
//pub const KEY_IGNORE_FILE3: &str = "IGNORE_FILE3";
//pub const DEFAULT_IGNORE_LIST: &str = "";

pub const KEY_CFG: &str = "cfg";
pub const KEY_CFG_SHORT: &str = "-c";
pub const KEY_CFG_LONG: &str = "--config";
pub const KEY_CFG_HELP: &str = "Specify cfg file";
pub const KEY_CFG_FILE: &str = "CFG_FILE";
pub const DEFAULT_CFG_FILENAME: &str = "linden.yml";

pub const KEY_DIR: &str = "directory";
pub const KEY_DIR_SHORT: &str = "-d";
pub const KEY_DIR_LONG: &str = "--directory";
pub const KEY_DIR_HELP: &str = "directory to create db in";
pub const KEY_DIR_FILENAME: &str = "DIRECTORY";
pub const DEFAULT_DIR_FILENAME: &str = ".";

pub const KEY_QUIET: &str = "quiet";
pub const KEY_QUIET_SHORT: &str = "-q";
pub const KEY_QUIET_LONG: &str = "--quiet";
pub const KEY_QUIET_HELP: &str = "be silent";

pub const KEY_VERBOSE: &str = "verbose";
pub const KEY_VERBOSE_SHORT: &str = "-v";
//pub const KEY_VERBOSE_LONG: &'static str = "--verbose";
pub const KEY_VERBOSE_HELP: &str = "Show additional information"; // TODO: text

impl<'a, 'b> Cli<'a, 'b> {
    // XXX: mb return option is better?
    pub fn get_bitflag(matches: &ArgMatches) -> usize {
        let mut bitflag = NODE_NONE;

        if matches.is_present(KEY_BITFLAG_NAME) {
            bitflag |= NODE_NAME;
        };
        if matches.is_present(KEY_BITFLAG_DESC) {
            bitflag |= NODE_DESC;
        };
        if matches.is_present(KEY_BITFLAG_ACCESSED) {
            bitflag |= NODE_ACCESSED;
        };
        if matches.is_present(KEY_BITFLAG_MODIFIED) {
            bitflag |= NODE_MODIFIED;
        };
        if matches.is_present(KEY_BITFLAG_CREATED) {
            bitflag |= NODE_CREATED;
        };
        if matches.is_present(KEY_BITFLAG_SIZE) {
            bitflag |= NODE_SIZE;
        };
        if matches.is_present(KEY_BITFLAG_FILE_TYPE) {
            bitflag |= NODE_FILE_TYPE;
        };
        if matches.is_present(KEY_BITFLAG_TAGS) {
            bitflag |= NODE_TAGS;
        };
        if matches.is_present(KEY_BITFLAG_CHILDREN) {
            bitflag |= NODE_CHILDREN;
        };

        bitflag
    }
}
