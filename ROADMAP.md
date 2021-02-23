# Roadmap


- deb packet?
- config ignore for every entry?
- ignore by sha2, time etc?
- Dry run option for all commands
    - `clear`

- List of args letters, check conflicts?
- filter + template for output
- adding by regexp

- regexp for file name

- Accept borrow and own `T: AsRef<>`

- size minification
    - templating, tera ~7 MB



Targets:
- trying universal application structure

- ask if file description recognition needed (e.g. Cargo.toml)

- Tree name as project name
    - [x] linden
    - [ ] aspen
    - [ ] alder

- Logo: linden leaf


CLI:

- high level commands (multiple nodes access, global changes, error check):
    - `init` - initialize db
    - `clean` - clean db, only existing files stay?

    - `rm` - remove file or path
    - `add` - add file or path (`--d description`)

    - `clear` - clear file or path fields
    - `update` - update from existing files
    - `status` - current status

    - `make` - ?
    - `search`
    - `diff`
    - `find`
    - `exists`
    - `check` - ?
    - `duplicates` - ?
    - `fill [-b <DB>]` - fill fields with given value

- mid level
    - move = read, create, delete
    - rename = update (create, delete)

- low level commands (single node access, local changes), CRUD?:
    - create
    - read
    - update
    - delete

- fields and args global view (conflicts?):
    - `-d, --description` - description
    - `-c, --comment` - description
    - `-g, --tags` - tags
    - `-d, --description` - description
    - `-a, --accessed` - accessed time
    - `-m, --modified` - modified time
    - `-c, --created` - created time
    - `-s, --size` - size
    - `-t, --type` - type

    - `-R, --recursive` - recursive run command
    - `-f, --force` - force

    - `-E, --regex` - use regex instead of exact matching

- `-h, --help` - help
- `-q, --quiet` - be silent
- `-V, --version` - version
- `-v[vvvvv]` - verbose level (default: `-v`, max. `-vvvvv`)
- `-l [LOG_FILE] -vvvvv` - show log, show and save into file

- `init [-b <DB>] [-f] [-i] [ROOT_DIRECTORY]` - Initialize tree
    - `-b <DB>, --database=<DB>` - Specify database file (default: `.tree.yml`)
    - `-i <IGNORE_FILE1> [IGNORE_FILE2] ..., --ignore=<IGNORE_FILE1> [IGNORE_FILE2] ...` - Ignore list
    - `-f, --force` - Overwrite existing database file, if it already exists
    - `[ROOT_DIRECTORY]` - root directory (default: `.`)

    - `-a, --all`
    - `-D, --default`

    - `-N, --name`
    - `-T, --type`
    - `-S, --size`
    - `-C, --created`
    - `-M, --modified`
    - `-A, --accessed`
    - `-R, --recurse` = `-H, --children`

    - `-D, --description`
    - `-H, --hidden`
    - `-G, --tags=[TAG1]`
    - `-C, --comment`


- `fill [-b <DB>]` - fill fields with given value

- `add-ignore` - add ignore list
- `remove-ignore` - remove ignore list
- `edit-ignore` - edit ignore list
- `list-ignore` - list ignore list

- `update [<DIR>] [-s <STORAGE>]` - update/merge tree
    - `-f, --from` - compare from file to file

- `merge`

- `check [<DIR>] [-s <STORAGE>]` - check changes

- `compare`
    - `-f, --from` - compare from file to file

- `default [-s <STORAGE>] [-damcst]` - fill defaults
    - `-d, --description` - set description from defaults
    - `-a, --accessed` - set accessed time current time
    - `-m, --modified` - set modified time current time
    - `-c, --created` - set created time current time
    - `-s, --size` - size
    - `-t, --type` - type

- `status [-s <DB>] [FILE]`

- `export <FILE> [-t txt]`
    - `-t, --type [txt|html|md]` - type of output

- `-o, --output` - output file

- `print [-f "format"]`

- `create <FILE> [-s <STORAGE>]`
    - `-s, --storage` - specify tree storage file

- `edit <FILE> [-s <STORAGE>]` - edit tree entry
    - `-s, --storage` - specify tree storage file
    - `-d, --description` - description
    - `-a, --accessed` - accessed time
    - `-m, --modified` - modified time
    - `-c, --created` - created time
    - `-s, --size` - size
    - `-t, --type` - type

- `delete <FILE> [-s <STORAGE>]` - delete entry
- `rm <FILE>` - remove file from db
- `ls <DIR>`
- `cd <DIR>`


## File

what if?

- rename:
    - file name changed
    - file sha1 not changed
    - path not changed
    assume:
    - creation, modification, access analyse
    - size analyse

- edit:
    - file modify date changed
    - file len changed

- delete:
    - file not found

- move:
    - file name changed
    - path changed

- create:
    - file new name
    - new len
    - date


# Problems Solving & Decision Making

- lineal structure (vec) or tree?
    - [x] tree, human r/w
    - [ ] list tree

- db vs plain text
    - [x] plain text, human r/w
    - [ ] sql db, as an option?

- Tree, template parsing recursing calls how to solve?
    - [ ] manage this not in template not allows recursion, using inline in code instead

- Tree, too slow and recurse in struction, m.b. that's why?:
    - [x] Using Rc, RefCell
    - [ ] sha256 enabled by default

- Tree links broken
    - [ ] tabs and spaces differs

- use bitflags crate?
    - [ ] serialize, deserialize needed

- `BTreeMap` vs `Vec<BTreeMap>`
    - [ ] no number, random order
    - [ ] number order

- Templating
    - [ ] `tera` too large resulting executable, removed

- Type field
    - [x] Type field needed, directory and symlink - same and file and symlink - same

- child bitflags, and other fields like ignore from parent?
    - [ ] or, and, overwrite?

- Remove log:
    - [ ] minimize size?
    - [ ] no info?

- No interactivity on CLI
    - [x] common practice? scripting etc.
    - [ ] allowing to interact user

- Alias shortcut for some commands (`edit [PATH] -d [DESCRIPTION]` = `desc [DESCRIPTION] [PATH]`)
    - [ ] need to make sure what is what file and desc
