use crate::cli::constants::*;
use clap::ArgMatches;
use std::fmt;
use std::path::{Path, PathBuf};
use text_tree_elements::TextTreeElements;

use crate::file_tree::{FileTree, Node, NODE_DEFAULT, NODE_NONE, SORT_DSC};
use crate::file_tree::{
    NODE_ACCESSED, NODE_CHILDREN, NODE_CREATED, NODE_DESC, NODE_FILE_TYPE, NODE_MODIFIED,
    NODE_NAME, NODE_NOT_EXISTS, NODE_SIZE, NODE_TAGS,
};
use std::fs;

/// Command structure, used to save user given arguments (CLI args)
#[derive(Default, Debug)]
pub struct Command {
    /// `-b, --database <DB>` Database path (default: `.tree.yml`)
    pub db: PathBuf,
    /// `[PATH]` Target directory/file path (default: `.`)
    pub path: PathBuf,
    /// `-o, --output <OUTPUT>` Output file path
    pub output: PathBuf,
    /// `-t, --template` Template file path
    pub template: PathBuf,
    // ???
    //pub file: Option<PathBuf>,
    /// Bitflags
    pub bitflag: usize,
    /// -i, --ignore <FILE1> Ignore list
    pub ignore: Option<Vec<String>>,
    /// -r, --recursive Recursive processing
    pub recursive: bool,
    ///
    pub tags: Option<Vec<String>>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let db = self.db.to_str().unwrap();
        let path = self.path.to_str().unwrap();
        let output = self.output.to_str().unwrap();
        let bitflag = Node::bitflag_to_string(self.bitflag);
        //&self.ignore
        write!(
            f,
            "db: {} path: {} output: {} bitflag: {}",
            db, path, output, bitflag
        )
    }
}

impl Command {
    pub fn new() -> Self {
        Default::default()
    }

    // XXX: refactor self?
    // XXX: mb return option is better?
    pub fn get_bitflag(matches: &ArgMatches) -> Option<usize> {
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

        if bitflag != NODE_NONE {
            Some(bitflag)
        } else {
            None
        }
    }

    // XXX: functions getting parameters already converted from strings in acceptable format?
    //
    // XXX: prog init -o output.yml -cfg?
    pub fn init(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_INIT);

        // if db exists and no overwrite flag is given
        if self.db.exists() && !matches.is_present(KEY_FORCE) {
            error!("db already exists: {:?}", self.db);
            return;
        }

        // get working dir
        if !self.path.exists() {
            error!("file or path not exists: {:?}", self.path);
            return;
        }

        // init & save new tree from given path
        let node =
            Node::create_from_path_ext(&self.path, self.ignore.as_ref(), self.bitflag).unwrap();
        node.save(&self.db).unwrap();
    }

    /// show tracked/untracked/changed/removed paths/files
    pub fn status(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_STATUS);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        // get working dir
        if !self.path.exists() {
            error!("file or path not exists: {:?}", self.path);
            return;
        }

        // init text tree elements, branch and link
        let tree_elements = TextTreeElements::default();

        // open db and compare with current state
        if let Ok(mut node) = Node::load(&self.db) {
            node.fill_compare_status(
                None,
                &self.path,
                self.ignore.as_ref(),
                NODE_NAME | NODE_SIZE,
            )
            .unwrap();
            let rendered = node
                .process_template(&tree_elements, 0, 0, node.children_num(), "")
                .unwrap();
            let rendered = rendered.join("");

            // if output flag is given
            // NOTE: not using is_present() bc. default value is set
            //if matches.occurrences_of(KEY_OUTPUT) > 0 {
            // export
            //fs::write(&self.output, rendered.as_bytes()).unwrap();
            // if no output flag is given
            //} else {
            // print
            print!("{}", rendered);
            //}
        }
    }

    //// show tracked/untracked/changed/removed paths/files
    //pub fn status(&mut self, matches: &ArgMatches) {
    //self.get_args(&matches);
    //self.debug_msg(CMD_STATUS);

    //// get db file path
    //if !self.db.exists() {
    //error!("db not exists");
    //return;
    //}

    //let node =
    ////Node::create_from_path_ext(&self.path, self.ignore.as_ref(), self.bitflag).unwrap();
    //Node::create_from_path_ext(&self.path, self.ignore.as_ref(), NODE_NAME | NODE_SIZE).unwrap();
    //// export
    //if let Ok(origin) = Node::load(&self.db) {
    //Self::comparerer(&node, &origin);
    //let tree = Default::default();
    //let rendered = node
    //.process_template(&tree, 0, 0, node.children_num(), "", &origin)
    //.unwrap();
    //let mut rendered = rendered.join("");

    //// if output flag is given
    //// NOTE: not using is_present() bc. default value is set
    //if matches.occurrences_of(KEY_OUTPUT) > 0 {
    //// export
    //fs::write(&self.output, rendered.as_bytes()).unwrap();
    //// if no output flag is given
    //} else {
    //// print
    //print!("{}", rendered);
    //}
    //}
    //}

    // tree file, id, desc
    pub fn add(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_ADD);

        // db exists?
        if !self.db.exists() {
            error!("db not exists: {:?}", self.db);
            return;
        }

        // file exists?
        if !self.path.exists() {
            error!("file or path not exists");
            return;
        }

        // db open successed?
        if let Ok(mut node) = Node::load(&self.db) {
            if node.exists(&self.path) {
                error!("file or path is already tracking");
                return;
            }

            let child =
                Node::create_from_path_ext(&self.path, self.ignore.as_ref(), self.bitflag).unwrap();

            node.add_child(child.0);

            // add new path
            //node.add_path_ext(
            //Some(Node::to_weak(&node)),
            //&self.path,
            //self.ignore.as_ref(),
            //self.bitflag,
            //)
            //.unwrap();

            // save
            node.save(&self.db).unwrap();
        } else {
            error!("db read error");
            return;
        }
    }

    // tree file, id, desc
    pub fn edit(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_EDIT);

        // db exists?
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        // get description
        let desc = matches.value_of(KEY_DESC);
        // get comment
        let comment = matches.value_of(KEY_COMMENT);

        // file exists?
        if !self.path.exists() {
            error!("file or path not exists");
            return;
        }

        // db open successed?
        if let Ok(node) = Node::load(&self.db) {
            // get entry if exists
            if let Some(entry) = node.get(&self.path) {
                // modify description
                entry.borrow_mut().desc = desc.map(String::from);
                entry.borrow_mut().tags = self.tags.clone();
                entry.borrow_mut().comment = comment.map(String::from);
            } else {
                error!(
                    "path not found in db: {:?} (file or path not exists)",
                    &self.path
                );
            }
            // save
            node.save(&self.db).unwrap();
        } else {
            error!("db read error");
            return;
        }
    }

    pub fn merge(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_MERGE);

        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        unimplemented!();
    }

    pub fn update(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_UPDATE);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        //let path = PathBuf::from(matches.value_of(KEY_DIR).unwrap_or(DEFAULT_DIR_FILENAME));
    }

    // open db
    //pub fn open<P: AsRef<Path>>(path: P) {
    //// get db file path
    //let db = PathBuf::from()

    //// db exists?
    //if !db.exists() {
    //error!("db not exists");
    //return;
    //}

    //if let Ok(file_tree) = FileTree::read(&db) {}
    //}

    // read and show entry
    pub fn read(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_READ);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        if let Ok(node) = Node::load(&self.db) {
            // get entry if exists
            if let Some(entry) = node.get(&self.path) {
                println!(
                    "{:?} {}",
                    Node::get_full_path(&entry.borrow()),
                    entry.to_string_ext(self.bitflag)
                );
            } else {
                error!("file not found");
            }
        } else {
            error!("file tree read error");
        }
    }

    /// remove file/path
    /// rm [PATH] -R
    pub fn rm(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_RM);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        if let Ok(node) = Node::load(&self.db) {
            // get entry if exists
            if !node.exists(&self.path) {
                error!("path not exists");
                return;
            }

            if node.remove(&self.path).is_some() {
                node.save(&self.db).unwrap();
            } else {
                error!("cant remove");
            }
        } else {
            error!("cant load db");
        }
    }

    /// ls
    pub fn ls(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_LS);

        // db exists?
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        if let Ok(node) = Node::load(&self.db) {
            //let rendered = tree
            if let Err(err) = node.ls(&self.path) {
                error!("{}", err);
            }
        }
    }

    pub fn print(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_PRINT);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        let origin =
            Node::create_from_path_ext(&self.path, self.ignore.as_ref(), self.bitflag).unwrap();

        // export
        if let Ok(node) = Node::load(&self.db) {
            let tree = TextTreeElements::default();
            let rendered = node
                .process_template(&tree, 0, 0, node.children_num(), "")
                .unwrap();
            let rendered = rendered.join("");

            // if output flag is given
            // NOTE: not using is_present() bc. default value is set
            if matches.occurrences_of(KEY_OUTPUT) > 0 {
                // export
                fs::write(&self.output, rendered.as_bytes()).unwrap();
            // if no output flag is given
            } else {
                // print
                print!("{}", rendered);
            }
        }
    }

    //pub fn cmd_default(_storage: &Path, wordlist: Option<BTreeMap<String, String>>) {
    //if let Some(_wordlist) = wordlist {}
    //unimplemented!();
    //}

    // TODO: implement
    pub fn print_meta(db: &Path) {
        if let Ok(_node) = Node::load(&db) {
            //if let Some(ignore_list) = file_tree.ignore {
            //println!("ignore list:");
            //for file in ignore_list {
            //println!(" {}", file);
            //}
            //}
        }
    }

    // TODO: implement
    pub fn remove_ignore(db: &Path, _name: &str) {
        if let Ok(_node) = Node::load(&db) {
            //if let Some(ignore_list) = &mut node.ignore {
            //let i = ignore_list.iter().position(|r| r == name).unwrap();
            //ignore_list.remove(i);
            //file_tree.write(db).unwrap();
            //}
        }
    }

    pub fn edit_ignore(db: &Path, name: &str, new_name: &str) {
        if let Ok(mut file_tree) = FileTree::read(db) {
            if let Some(ignore_list) = &mut file_tree.ignore {
                let i = ignore_list.iter().position(|r| r == name).unwrap();
                //ignore_list.get(i).unwrap() = &new_name.to_string();
                // NOTE: out of bounds
                ignore_list[i] = new_name.to_string();
                file_tree.write(db).unwrap();
            }
        }
    }

    pub fn add_ignore(db: &Path, name: &str) {
        if let Ok(mut file_tree) = FileTree::read(db) {
            if let Some(ignore_list) = &mut file_tree.ignore {
                ignore_list.push(name.to_string());
                file_tree.write(db).unwrap();
            }
        }
    }

    // XXX: sorting by multiple fields
    // not bitflags but enum?
    pub fn sort(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_SORT);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        // get bitflags
        // FIXME: add sorting bitflags

        // clear and write
        // XXX: do not write dry run option
        if let Ok(node) = Node::load(&self.db) {
            node.sort_ext(self.bitflag);
            node.save(&self.db).unwrap();
        }
    }

    // XXX: add path support and children flag
    /// clear fields for path
    pub fn clear(&mut self, matches: &ArgMatches) {
        self.get_args(&matches);
        self.debug_msg(CMD_CLEAR);

        // get db file path
        if !self.db.exists() {
            error!("db not exists");
            return;
        }

        // clear and write
        if let Ok(tree) = Node::load(&self.db) {
            if let Some(node) = tree.get(&self.path) {
                node.clear_ext(self.bitflag, self.recursive);
                tree.save(&self.db).unwrap();
            }
        }
    }

    /// print current command and it's args
    pub fn debug_msg(&self, cmd: &str) {
        debug!("command: {} args: {}", cmd, &self,);
    }

    /// store user given options
    pub fn get_args(&mut self, matches: &ArgMatches) {
        // db file
        self.db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get given path (-c PATH)
        self.path = PathBuf::from(matches.value_of(KEY_PATH).unwrap_or("."));
        // get field bitflags
        self.bitflag = Command::get_bitflag(matches).unwrap_or(NODE_NONE);
        // if empty nodes
        if matches.is_present(KEY_EMPTY) {
            self.bitflag = NODE_NONE;
        }
        // get template file path
        self.template = PathBuf::from(
            matches
                .value_of(KEY_TEMPLATE)
                .unwrap_or(DEFAULT_TEMPLATE_FILENAME),
        );
        // get output file path
        self.output = PathBuf::from(
            matches
                .value_of(KEY_OUTPUT)
                .unwrap_or(DEFAULT_OUTPUT_FILENAME),
        );
        // file
        //self.file = matches.value_of(KEY_FILE_NAME).map(|p| PathBuf::from(p));

        // get description
        self.recursive = matches.is_present(KEY_RECURSIVE);

        // get ignore list
        let v: Vec<String>;
        self.ignore = match matches.values_of(KEY_IGNORE) {
            Some(files) => {
                v = files.map(|s| s.to_string()).collect();
                Some(v)
            }
            None => None,
        };

        // get tags
        let v: Vec<String>;
        self.tags = match matches.values_of(KEY_TAGS) {
            Some(t) => {
                v = t.map(|s| s.to_string()).collect();
                Some(v)
            }
            None => None,
        };
    }

    /// match against command
    pub fn match_command(&mut self, matches: &ArgMatches) {
        match matches.subcommand() {
            (CMD_INIT, Some(matches)) => self.init(matches),
            (CMD_LS, Some(matches)) => self.ls(matches),
            (CMD_EDIT, Some(matches)) => self.edit(matches),
            (CMD_READ, Some(matches)) => self.read(matches),
            (CMD_RM, Some(matches)) => self.rm(matches),
            (CMD_STATUS, Some(matches)) => self.status(matches),
            (CMD_PRINT, Some(matches)) => self.print(matches),
            (CMD_SORT, Some(matches)) => self.sort(matches),
            (CMD_CLEAR, Some(matches)) => self.clear(matches),
            (CMD_UPDATE, Some(matches)) => self.update(matches),
            (CMD_ADD, Some(matches)) => self.add(matches),
            (CMD_MERGE, Some(matches)) => self.merge(matches),
            _ => info!("{}", LOG_CLI_NO_SUBCOMMAND_RECIEVED),
        }
    }
}
