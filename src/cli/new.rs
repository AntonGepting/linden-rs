use super::cli::Cli;
use super::constants::*;
use crate::application::Application;
use clap::{App, Arg, SubCommand};

// XXX: mb functions for every command, and mb args
impl<'a, 'b> Cli<'a, 'b> {
    // create db arg
    // `-b, --database .tree.yml`
    pub fn arg_db() -> Arg<'a, 'b> {
        Arg::with_name(KEY_DB)
            .short(KEY_DB_SHORT)
            .long(KEY_DB_LONG)
            .value_name(KEY_DB_FILE)
            .default_value(DEFAULT_DB_FILENAME)
            .help(KEY_DB_HELP)
            .takes_value(true)
    }

    // create path arg
    // `[PATH]`
    pub fn arg_path() -> Arg<'a, 'b> {
        Arg::with_name(KEY_PATH)
            .help(KEY_PATH_HELP)
            .default_value(DEFAULT_PATH)
            .index(1)
    }

    /// name as bitflag
    pub fn arg_bitflag_name() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_NAME)
            .short(KEY_BITFLAG_NAME_SHORT)
            .long(KEY_BITFLAG_NAME_LONG)
            .help(KEY_BITFLAG_NAME_HELP)
    }

    /// desc as bitflag
    pub fn arg_bitflag_desc() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_DESC)
            .short(KEY_BITFLAG_DESC_SHORT)
            .long(KEY_BITFLAG_DESC_LONG)
            .help(KEY_BITFLAG_DESC_HELP)
    }

    /// accessed time as bitflag
    pub fn arg_bitflag_accessed() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_ACCESSED)
            .short(KEY_BITFLAG_ACCESSED_SHORT)
            .long(KEY_BITFLAG_ACCESSED_LONG)
            .help(KEY_BITFLAG_ACCESSED_HELP)
    }

    /// created time as bitflag
    pub fn arg_bitflag_created() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_CREATED)
            .short(KEY_BITFLAG_CREATED_SHORT)
            .long(KEY_BITFLAG_CREATED_LONG)
            .help(KEY_BITFLAG_CREATED_HELP)
    }

    /// modified time as bitflag
    pub fn arg_bitflag_modified() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_MODIFIED)
            .short(KEY_BITFLAG_MODIFIED_SHORT)
            .long(KEY_BITFLAG_MODIFIED_LONG)
            .help(KEY_BITFLAG_MODIFIED_HELP)
    }

    /// size as bitflag
    pub fn arg_bitflag_size() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_SIZE)
            .short(KEY_BITFLAG_SIZE_SHORT)
            .long(KEY_BITFLAG_SIZE_LONG)
            .help(KEY_BITFLAG_SIZE_HELP)
    }

    /// type as bitflag
    pub fn arg_bitflag_type() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_FILE_TYPE)
            .short(KEY_BITFLAG_FILE_TYPE_SHORT)
            .long(KEY_BITFLAG_FILE_TYPE_LONG)
            .help(KEY_BITFLAG_FILE_TYPE_HELP)
    }

    /// tags as bitflag
    pub fn arg_bitflag_tags() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_TAGS)
            .short(KEY_BITFLAG_TAGS_SHORT)
            .long(KEY_BITFLAG_TAGS_LONG)
            .help(KEY_BITFLAG_TAGS_HELP)
    }

    /// comment as bitflag
    pub fn arg_bitflag_comment() -> Arg<'a, 'b> {
        Arg::with_name(KEY_BITFLAG_COMMENT)
            .short(KEY_BITFLAG_COMMENT_SHORT)
            .long(KEY_BITFLAG_COMMENT_LONG)
            .help(KEY_BITFLAG_COMMENT_HELP)
    }

    // prepare and use clap functions for cli
    pub fn new(application: Application) -> Self {
        let app = App::new(APP_NAME)
            .version(APP_VERSION)
            .about(APP_ABOUT)
            .author(APP_AUTHOR)
            .arg(
                Arg::with_name(KEY_LOG)
                    .short(KEY_LOG_SHORT)
                    .long(KEY_LOG_LONG)
                    .value_name(KEY_LOG_FILE)
                    .takes_value(true)
                    .default_value(DEFAULT_LOG_FILENAME)
                    .help(KEY_LOG_HELP),
            )
            .arg(
                Arg::with_name(KEY_CFG)
                    .short(KEY_CFG_SHORT)
                    .long(KEY_CFG_LONG)
                    .value_name(KEY_CFG_FILE)
                    .takes_value(true)
                    .default_value(DEFAULT_CFG_FILENAME)
                    .help(KEY_CFG_HELP),
            )
            .arg(
                Arg::with_name(KEY_OUTPUT)
                    .short(KEY_OUTPUT_SHORT)
                    .long(KEY_OUTPUT_LONG)
                    .value_name(KEY_OUTPUT_FILE)
                    .takes_value(true)
                    .default_value(DEFAULT_OUTPUT_FILENAME)
                    .help(KEY_OUTPUT_HELP),
            )
            .arg(
                Arg::with_name(KEY_QUIET)
                    .short(KEY_QUIET_SHORT)
                    .long(KEY_QUIET_LONG)
                    .help(KEY_QUIET_HELP),
            )
            .arg(
                Arg::with_name(KEY_VERBOSE)
                    .short(KEY_VERBOSE_SHORT)
                    .multiple(true)
                    //.takes_value(false)
                    .help(KEY_VERBOSE_HELP),
            )
            // ls
            .subcommand(
                SubCommand::with_name(CMD_LS)
                    .about(CMD_LS_TEXT)
                    .arg(Cli::arg_db())
                    .arg(Cli::arg_path()),
            )
            // sort
            .subcommand(
                SubCommand::with_name(CMD_SORT)
                    .about(CMD_SORT_TEXT)
                    .arg(Cli::arg_db())
                    .arg(Cli::arg_bitflag_name())
                    .arg(Cli::arg_bitflag_desc())
                    .arg(Cli::arg_bitflag_accessed())
                    .arg(Cli::arg_bitflag_created())
                    .arg(Cli::arg_bitflag_modified())
                    .arg(Cli::arg_bitflag_size())
                    .arg(Cli::arg_bitflag_type())
                    .arg(Cli::arg_bitflag_tags())
                    .arg(Cli::arg_bitflag_comment())
                    .arg(
                        Arg::with_name(KEY_BITFLAG_SORT_ORDER)
                            .short(KEY_BITFLAG_SORT_ORDER_SHORT)
                            .long(KEY_BITFLAG_SORT_ORDER_LONG)
                            .help(KEY_BITFLAG_SORT_ORDER_HELP)
                            .takes_value(true)
                            .possible_value(KEY_BITFLAG_SORT_ORDER_ASC)
                            .possible_value(KEY_BITFLAG_SORT_ORDER_DESC),
                    ),
            )
            // clear
            .subcommand(
                SubCommand::with_name(CMD_CLEAR)
                    .about(CMD_CLEAR_TEXT)
                    .arg(Cli::arg_db())
                    //.arg(
                    //Arg::with_name(KEY_DIR)
                    //.help(KEY_DIR_HELP)
                    //.default_value(DEFAULT_DIR_FILENAME)
                    //.index(1),
                    //)
                    .arg(Cli::arg_path())
                    .arg(
                        Arg::with_name(KEY_RECURSIVE)
                            .short(KEY_RECURSIVE_SHORT)
                            .long(KEY_RECURSIVE_LONG)
                            .help(KEY_RECURSIVE_HELP),
                    )
                    .arg(
                        Arg::with_name(KEY_BITFLAG_NAME)
                            .short(KEY_BITFLAG_NAME_SHORT)
                            .long(KEY_BITFLAG_NAME_LONG)
                            .help(KEY_BITFLAG_NAME_HELP),
                    )
                    .arg(Cli::arg_bitflag_desc())
                    .arg(Cli::arg_bitflag_accessed())
                    .arg(Cli::arg_bitflag_created())
                    .arg(Cli::arg_bitflag_modified())
                    .arg(Cli::arg_bitflag_size())
                    .arg(Cli::arg_bitflag_type())
                    .arg(Cli::arg_bitflag_tags())
                    .arg(Cli::arg_bitflag_comment())
                    .arg(
                        Arg::with_name(KEY_BITFLAG_CHILDREN)
                            .short(KEY_BITFLAG_CHILDREN_SHORT)
                            .long(KEY_BITFLAG_CHILDREN_LONG)
                            .help(KEY_BITFLAG_CHILDREN_HELP),
                    ),
            )
            // init
            .subcommand(
                SubCommand::with_name(CMD_INIT)
                    .about(CMD_INIT_TEXT)
                    .arg(Cli::arg_db())
                    .arg(
                        Arg::with_name(KEY_IGNORE)
                            .short(KEY_IGNORE_SHORT)
                            .long(KEY_IGNORE_LONG)
                            .help(KEY_IGNORE_HELP)
                            //.value_names(&[KEY_IGNORE_FILE1, KEY_IGNORE_FILE2, KEY_IGNORE_FILE3])
                            .takes_value(true)
                            //.number_of_values(1)
                            .multiple(true),
                        //.min_values(1),
                    )
                    .arg(Cli::arg_bitflag_name())
                    .arg(Cli::arg_bitflag_desc())
                    .arg(Cli::arg_bitflag_accessed())
                    .arg(Cli::arg_bitflag_created())
                    .arg(Cli::arg_bitflag_modified())
                    .arg(Cli::arg_bitflag_size())
                    .arg(Cli::arg_bitflag_type())
                    .arg(Cli::arg_bitflag_tags())
                    .arg(Cli::arg_bitflag_comment())
                    // -f, --force
                    .arg(
                        Arg::with_name(KEY_FORCE)
                            .short(KEY_FORCE_SHORT)
                            .long(KEY_FORCE_LONG)
                            .help(KEY_FORCE_HELP),
                    )
                    // -E?
                    .arg(
                        Arg::with_name(KEY_EMPTY)
                            .short(KEY_EMPTY_SHORT)
                            .long(KEY_EMPTY_LONG)
                            .help(KEY_EMPTY_HELP),
                    )
                    //.arg(
                    //Arg::with_name(KEY_DIR)
                    //.short(KEY_DIR_SHORT)
                    //.long(KEY_DIR_LONG)
                    //.help(KEY_DIR_HELP)
                    //.value_name(KEY_DIR_FILENAME)
                    //.default_value(DEFAULT_DIR_FILENAME)
                    //.takes_value(true),
                    //),
                    //.arg(
                    //Arg::with_name(KEY_DIR)
                    //.help(KEY_DIR_HELP)
                    //.default_value(DEFAULT_DIR_FILENAME)
                    //.index(1),
                    //),
                    .arg(Cli::arg_path()),
            )
            // add
            .subcommand(
                SubCommand::with_name(CMD_ADD)
                    .about(CMD_ADD_TEXT)
                    .arg(Cli::arg_db())
                    // -R, --recursive
                    .arg(
                        Arg::with_name(KEY_RECURSIVE)
                            .short(KEY_RECURSIVE_SHORT)
                            .long(KEY_RECURSIVE_LONG)
                            .help(KEY_RECURSIVE_HELP),
                    )
                    .arg(Cli::arg_path()),
            )
            // edit
            .subcommand(
                SubCommand::with_name(CMD_EDIT)
                    .about(CMD_EDIT_TEXT)
                    .arg(Cli::arg_db())
                    .arg(
                        Arg::with_name(KEY_DESC)
                            .short(KEY_DESC_SHORT)
                            .long(KEY_DESC_LONG)
                            .value_name(KEY_DESCRIPTION)
                            .help(KEY_DESC_HELP)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name(KEY_TAGS)
                            .short(KEY_TAGS_SHORT)
                            .long(KEY_TAGS_LONG)
                            //.value_name(KEY_TAGS)
                            .help(KEY_TAGS_HELP)
                            .takes_value(true)
                            .multiple(true),
                    )
                    .arg(
                        Arg::with_name(KEY_COMMENT)
                            .short(KEY_COMMENT_SHORT)
                            .long(KEY_COMMENT_LONG)
                            .value_name(KEY_COMMENT_DESC)
                            .help(KEY_COMMENT_HELP)
                            .takes_value(true),
                    )
                    .arg(Cli::arg_path()),
            )
            // status
            .subcommand(
                SubCommand::with_name(CMD_STATUS)
                    .about(CMD_STATUS_TEXT)
                    .arg(Cli::arg_db()),
            )
            // rm
            .subcommand(
                SubCommand::with_name(CMD_RM)
                    .about(CMD_RM_TEXT)
                    .arg(Cli::arg_db())
                    .arg(
                        Arg::with_name(KEY_DESC)
                            .short(KEY_DESC_SHORT)
                            .long(KEY_DESC_LONG)
                            .value_name(KEY_DESCRIPTION)
                            .help(KEY_DESC_HELP)
                            .takes_value(true),
                    )
                    .arg(Cli::arg_path()),
            )
            // update
            .subcommand(
                SubCommand::with_name(CMD_UPDATE)
                    .about(CMD_UPDATE_TEXT)
                    .arg(Cli::arg_db())
                    .arg(Cli::arg_path()),
            )
            // read
            .subcommand(
                SubCommand::with_name(CMD_READ)
                    .about(CMD_READ_TEXT)
                    .arg(Cli::arg_db())
                    .arg(
                        Arg::with_name(KEY_DESC)
                            .short(KEY_DESC_SHORT)
                            .long(KEY_DESC_LONG)
                            .value_name(KEY_DESCRIPTION)
                            .help(KEY_DESC_HELP)
                            .takes_value(true),
                    )
                    .arg(Cli::arg_path()),
            )
            // print
            .subcommand(
                SubCommand::with_name(CMD_PRINT)
                    .about(CMD_PRINT_TEXT)
                    .arg(Cli::arg_db())
                    .arg(
                        Arg::with_name(KEY_TEMPLATE)
                            .short(KEY_TEMPLATE_SHORT)
                            .long(KEY_TEMPLATE_LONG)
                            .value_name(KEY_TEMPLATE_FILE)
                            .help(KEY_TEMPLATE_HELP)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name(KEY_OUTPUT)
                            .short(KEY_OUTPUT_SHORT)
                            .long(KEY_OUTPUT_LONG)
                            .default_value(DEFAULT_OUTPUT_FILENAME)
                            .value_name(KEY_OUTPUT_FILE)
                            .takes_value(true)
                            .help(KEY_OUTPUT_HELP),
                    ),
            )
            // meta
            .subcommand(
                SubCommand::with_name(CMD_PRINT_META)
                    .about(CMD_PRINT_META_TEXT)
                    .arg(Cli::arg_db()),
            );

        Cli {
            cli: app,
            app: application,
            //matches: app.get_matches(),
        }
    }
}
