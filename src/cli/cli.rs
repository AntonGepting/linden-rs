use crate::application::Application;
use crate::cli::common::*;
use crate::error::Error;
use crate::mylog::Log;
use clap::{App, ArgMatches};
//use file_tree::tree_entry::tree_entry::{COMPARE_ASC, COMPARE_DSC};

use crate::file_tree::{FileTree, Node, NODE_DEFAULT, NODE_NONE};
use log::LevelFilter;
use std::fs;
use std::path::{Path, PathBuf};

//#[derive(Default, Debug)]
pub struct Cli<'a, 'b> {
    pub cli: App<'a, 'b>,
    pub app: Application,
}

impl<'a, 'b> Cli<'a, 'b> {
    //pub fn debug() {
    //}

    // XXX: functions getting parameters already converted from strings in acceptable format?
    //
    // XXX: prog init -o output.yml -cfg?
    pub fn cmd_init(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get working dir
        //let path = PathBuf::from(".");
        let path = PathBuf::from(matches.value_of(KEY_DIR).unwrap_or(DEFAULT_DIR_FILENAME));

        // get bitflags
        let mut bitflag = Cli::get_bitflag(matches);
        // if none given use defaults
        if bitflag == NODE_NONE {
            bitflag = NODE_DEFAULT;
        }

        // get ignore list
        //
        let files: Vec<String> = matches
            .values_of(KEY_IGNORE)
            .unwrap()
            .map(|s| s.to_string())
            .collect();
        let ignore_list = Some(&files);

        debug!(
            "recieved subcommand: {} {:?} {:?} {:?}",
            CMD_INIT,
            &path,
            &db,
            ignore_list.as_ref()
        );

        // if db not exists or overwrite flag is given, write new db
        if !db.exists() || matches.is_present(KEY_FORCE) {
            // init & save new tree from given path
            let file_tree = Node::create_from_path_ext(&path, ignore_list, bitflag).unwrap();
            file_tree.save(&db).unwrap();
        } else {
            error!("db already exists: {:?}", db);
        }
    }

    pub fn cmd_status(matches: &ArgMatches) {
        // get db file path
        let _db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
    }

    // tree file, id, desc
    pub fn cmd_add(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get description
        let desc = matches.value_of(KEY_RECOURSIVE);
        // get file
        if let Some(file) = matches.value_of(KEY_FILE_NAME) {
            let file = PathBuf::from(file);
            debug!("cmd_add({:?}, {:?}, {:?})", db, file, desc);

            // file exists?
            if !file.exists() {
                error!("file not exists");
                return;
            }

            // db exists?
            if !db.exists() {
                error!("db not exists");
                return;
            }

            // db open successed?
            if let Ok(node) = Node::load(&db) {
                // get entry if exists
                if let Some(_entry) = node.get(&file) {
                    // modify description
                    //entry.borrow_mut().desc = desc.map(String::from);
                } else {
                    println!("path not found in db: {:?} (file or path not exists)", file);
                }
            // save
            //node.save(&db).unwrap();
            } else {
                error!("db read error");
                return;
            }
        } else {
            error!("must specify file to add");
        }
    }

    // tree file, id, desc
    pub fn cmd_edit(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get description
        let desc = matches.value_of(KEY_DESC);
        // get file
        if let Some(file) = matches.value_of(KEY_FILE_NAME) {
            let file = PathBuf::from(file);
            debug!("cmd_edit({:?}, {:?}, {:?})", db, file, desc);

            // file exists?
            if !file.exists() {
                error!("file not exists");
                return;
            }

            // db exists?
            if !db.exists() {
                error!("db not exists");
                return;
            }

            // db open successed?
            if let Ok(node) = Node::load(&db) {
                // get entry if exists
                if let Some(entry) = node.get(&file) {
                    // modify description
                    entry.borrow_mut().desc = desc.map(String::from);
                } else {
                    println!("path not found in db: {:?} (file or path not exists)", file);
                }
                // save
                node.save(&db).unwrap();
            } else {
                error!("db read error");
                return;
            }
        } else {
            error!("must specify file to edit");
        }
    }

    pub fn cmd_merge(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        if !db.exists() {
            error!("db not exists");
            return;
        }
    }

    pub fn cmd_update(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get working dir
        //let path = PathBuf::from(".");
        let path = PathBuf::from(matches.value_of(KEY_DIR).unwrap_or(DEFAULT_DIR_FILENAME));
        // get ignore list
        let ignore_list: Option<Vec<String>> = if let Some(v) = matches.values_of(KEY_IGNORE) {
            Some(v.map(|s| s.to_string()).collect())
        } else {
            None
        };

        debug!(
            "recieved subcommand: {} {:?} {:?} {:?}",
            CMD_UPDATE, &path, &db, &ignore_list
        );

        if !db.exists() {
            error!("db not exists");
            return;
        }
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
    pub fn cmd_read(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // db exists?
        if !db.exists() {
            error!("db not exists");
            return;
        }

        // get file
        if let Some(file) = matches.value_of(KEY_FILE_NAME) {
            // get file name
            let file = PathBuf::from(file);

            debug!("cmd_show({:?}, {:?})", db, file);

            if let Ok(node) = Node::load(&db) {
                // get entry if exists
                if let Some(entry) = node.get(&file) {
                    let bitflag = Cli::get_bitflag(matches);
                    println!(
                        "{:?} {}",
                        Node::get_full_path(&entry.borrow()),
                        entry.to_string_ext(bitflag)
                    );
                } else {
                    error!("file not found");
                }
            } else {
                error!("file tree read error");
            }
        } else {
            error!("must specify file to read");
        }
    }

    pub fn cmd_delete(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));

        // get working dir
        if let Some(path) = matches.value_of(KEY_FILE_NAME) {
            let path = PathBuf::from(path);
            debug!("recieved subcommand: {} {:?} {:?}", CMD_DELETE, &path, &db);

            if let Ok(node) = Node::load(&db) {
                // get entry if exists
                //let rendered = tree
                //if let Err(err) = tree.ls(&path) {
                //error!("{}", err);
                //}
                //node.ls(Path::new("."));
                if node.remove(&path).is_some() {
                    // save
                    //node.ls(Path::new("."));
                    node.save(&db).unwrap();
                } else {
                    error!("cant remove");
                }
            } else {
                error!("you must specify path");
            }
        }
    }

    pub fn cmd_ls(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get working dir
        let path = PathBuf::from(matches.value_of(KEY_PATH).unwrap_or("."));
        debug!("recieved subcommand: {} {:?} {:?}", CMD_LS, &path, &db);

        if let Ok(node) = Node::load(&db) {
            //let rendered = tree
            if let Err(err) = node.ls(&path) {
                error!("{}", err);
            }
        }
    }

    pub fn cmd_print(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        // get template file path
        let template = PathBuf::from(
            matches
                .value_of(KEY_TEMPLATE)
                .unwrap_or(DEFAULT_TEMPLATE_FILENAME),
        );

        // export
        debug!("cmd_edit({:?})", db);
        if let Ok(node) = Node::load(&db) {
            let rendered = node
                .process_template(0, 0, node.children_num(), "", &template)
                .unwrap();

            // if output flag is given
            // NOTE: not using is_present() bc. default value is set
            if matches.occurrences_of(KEY_OUTPUT) > 0 {
                // get output file path
                let output = PathBuf::from(
                    matches
                        .value_of(KEY_OUTPUT)
                        .unwrap_or(DEFAULT_OUTPUT_FILENAME),
                );
                // export
                fs::write(output, rendered.as_bytes()).unwrap();
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
    pub fn cmd_print_meta(db: &Path) {
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
    pub fn cmd_remove_ignore(db: &Path, _name: &str) {
        if let Ok(_node) = Node::load(&db) {
            //if let Some(ignore_list) = &mut node.ignore {
            //let i = ignore_list.iter().position(|r| r == name).unwrap();
            //ignore_list.remove(i);
            //file_tree.write(db).unwrap();
            //}
        }
    }

    pub fn cmd_edit_ignore(db: &Path, name: &str, new_name: &str) {
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

    pub fn cmd_add_ignore(db: &Path, name: &str) {
        if let Ok(mut file_tree) = FileTree::read(db) {
            if let Some(ignore_list) = &mut file_tree.ignore {
                ignore_list.push(name.to_string());
                file_tree.write(db).unwrap();
            }
        }
    }

    // XXX: sorting by multiple fields
    // not bitflags but enum?
    pub fn cmd_sort(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));

        // get working dir
        let _path = PathBuf::from(matches.value_of(KEY_PATH).unwrap_or("."));

        // get bitflags
        let bitflag = Cli::get_bitflag(matches);

        debug!("recieved command: {} {:b}", CMD_SORT, bitflag);

        // clear and write
        // XXX: do not write dry run option
        if let Ok(node) = Node::load(&db) {
            node.sort_ext(bitflag);
            node.save(&db).unwrap();
        }
    }

    // XXX: add path support and children flag
    // clear
    pub fn cmd_clear(matches: &ArgMatches) {
        // get db file path
        let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));

        // get working dir
        let path = PathBuf::from(matches.value_of(KEY_PATH).unwrap_or("."));

        // get field
        let bitflag = Cli::get_bitflag(matches);

        debug!("recieved command: {} {:?} {:b}", CMD_CLEAR, path, bitflag);

        // clear and write
        // XXX: do not write dry run option
        if let Ok(node) = Node::load(&db) {
            node.clear_ext(bitflag);
            node.save(&db).unwrap();
        }
    }

    // check if any command found
    pub fn process_matches(mut self) -> Result<(), Error> {
        // process keys
        let matches = self.cli.get_matches();

        // verbose level
        self.app.log_level_filter = if matches.is_present(KEY_QUIET) {
            // silent
            // NOTE: clap is still reporting errors
            Some(LevelFilter::Off)
        } else {
            // verbose level
            match matches.occurrences_of(KEY_VERBOSE) {
                0 => None,
                1 => Some(LevelFilter::Error),
                2 => Some(LevelFilter::Warn),
                3 => Some(LevelFilter::Info),
                4 => Some(LevelFilter::Debug),
                5 => Some(LevelFilter::Trace),
                // XXX: print too many -v flags
                _ => Some(LevelFilter::Trace),
            }
        };

        // logging on?
        if matches.is_present(KEY_LOG) {
            let log_filename = matches.value_of(KEY_LOG).unwrap_or(DEFAULT_LOG_FILENAME);
            Log::create(log_filename, self.app.log_level_filter).unwrap();
            info!("{}: {}", "start logging", log_filename);
        }

        error!("error example");
        warn!("warn example");
        info!("info example");
        debug!("debug example");
        trace!("trace example");

        // load custom / create default app config
        if matches.is_present(KEY_CFG) {
            self.app.custom_cfg = matches.value_of(KEY_CFG).map(PathBuf::from);
            //match Config::load(self.app.custom_cfg.as_ref()) {
            //Ok(config) => self.app.config = config,
            //Err(e) => error!("{}: {}", LOG_CLI_CFG_ERROR, e.message),
        };

        //if let Some(matches) = matches.subcommand_matches(Cli::CMD_START) {
        //self.app.project_path = PathBuf::from(
        //matches
        //.value_of(Cli::KEY_PROJECT)
        //.unwrap_or(Cli::DEFAULT_PROJECT_FILENAME),
        //);
        //info!(
        //"{}: {} {:?}",
        //LOG_CLI_SUBCOMMAND_RECIEVED,
        //Cli::CMD_START,
        //&self.app.project_path
        //);
        //self.app.start()?;
        //}

        // check for given command
        match matches.subcommand() {
            (CMD_INIT, Some(matches)) => Cli::cmd_init(matches),
            (CMD_LS, Some(matches)) => Cli::cmd_ls(matches),
            (CMD_EDIT, Some(matches)) => Cli::cmd_edit(matches),
            (CMD_READ, Some(matches)) => Cli::cmd_read(matches),
            (CMD_DELETE, Some(matches)) => Cli::cmd_delete(matches),
            (CMD_STATUS, Some(matches)) => Cli::cmd_status(matches),
            (CMD_PRINT, Some(matches)) => Cli::cmd_print(matches),
            (CMD_SORT, Some(matches)) => Cli::cmd_sort(matches),
            (CMD_CLEAR, Some(matches)) => Cli::cmd_clear(matches),
            _ => info!("{}", LOG_CLI_NO_SUBCOMMAND_RECIEVED),
        }

        // edit
        //if let Some(matches) = matches.subcommand_matches(CMD_EDIT) {
        //// get db file path
        //let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        //// get description
        //let desc = matches.value_of(KEY_DESC);
        //debug!("recieved subcommand: {} {:?}", CMD_EDIT, desc);

        //// get selected file
        //if let Some(s) = matches.value_of(KEY_FILE_NAME) {
        //let file = PathBuf::from(s);
        //debug!("recieved subcommand: {} {:?}", CMD_EDIT, &file);
        //Cli::subcommand_edit(&db, &file, desc);
        //} else {
        //}

        // read
        //if let Some(matches) = matches.subcommand_matches(CMD_READ) {
        //// get db file path
        //let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        //// get description
        //let desc = matches.value_of(KEY_DESC);
        //debug!("recieved subcommand: {} {:?}", CMD_READ, desc);

        //// get selected file
        //if let Some(s) = matches.value_of(KEY_FILE_NAME) {
        //let file = PathBuf::from(s);
        //debug!("recieved subcommand: {} {:?}", CMD_READ, &file);
        //Cli::subcommand_read(&db, &file);
        //} else {
        //}

        // export
        //} else if let Some(matches) = matches.subcommand_matches(CMD_PRINT) {
        //debug!("recieved subcommand: {}", CMD_PRINT);
        //// get db file path
        //let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        //// get template file path
        //let template = PathBuf::from(
        //matches
        //.value_of(KEY_TEMPLATE)
        //.unwrap_or(DEFAULT_TEMPLATE_FILENAME),
        //);
        //// get output file path
        //let output = PathBuf::from(
        //matches
        //.value_of(KEY_OUTPUT)
        //.unwrap_or(DEFAULT_OUTPUT_FILENAME),
        //);
        //// export
        //Cli::subcommand_print(&db, &output, &template);

        // print
        //if let Some(matches) = matches.subcommand_matches(CMD_PRINT_META) {
        //debug!("recieved subcommand: {}", CMD_PRINT_META);
        //// get db file path
        //let db = PathBuf::from(matches.value_of(KEY_DB).unwrap_or(DEFAULT_DB_FILENAME));
        //// print
        //Cli::subcommand_print_meta(&db);

        ////if let Some(matches) = matches.subcommand_matches(Cli::CMD_START) {
        //self.app.project_path = PathBuf::from(
        //matches
        //.value_of(Cli::KEY_PROJECT)
        //.unwrap_or(Cli::DEFAULT_PROJECT_FILENAME),
        //);
        //info!(
        //"{}: {} {:?}",
        //LOG_CLI_SUBCOMMAND_RECIEVED,
        //Cli::CMD_START,
        //&self.app.project_path
        //);
        //self.app.start()?;

        //// new project
        //} else if let Some(matches) = matches.subcommand_matches(Cli::CMD_NEW) {
        //self.app.project_path = PathBuf::from(
        //matches
        //.value_of(Cli::KEY_PROJECT)
        //.unwrap_or(Cli::DEFAULT_PROJECT_FILENAME),
        //);
        //info!(
        //"{}: {} {:?}",
        //LOG_CLI_SUBCOMMAND_RECIEVED,
        //Cli::CMD_NEW,
        //&self.app.project_path
        //);
        ////project.new(&project_path);

        //// ls projects
        //} else if let Some(_matches) = matches.subcommand_matches(Cli::CMD_LS) {
        //info!("{}: {}", LOG_CLI_SUBCOMMAND_RECIEVED, Cli::CMD_LS);
        //self.app.ls()?;

        //// save project
        //} else if let Some(_matches) = matches.subcommand_matches(Cli::CMD_SAVE) {
        //info!("{}: {}", LOG_CLI_SUBCOMMAND_RECIEVED, Cli::CMD_SAVE);
        ////Cli::save_project();

        //// default
        //} else {
        //info!("{}", LOG_CLI_NO_SUBCOMMAND_RECIEVED);
        //}

        //// exit msg
        info!("{}", LOG_CLI_END);

        Ok(())
    }
}
