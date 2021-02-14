# Roadmap

CLI:

- `-h, --help` - help
- `-q, --quiet` - be silent
- `-V, --version` - version
- `-v[vvvvv]` - verbose level (default: `-v`, max. `-vvvvv`)
- `-l [LOG_FILE] -vvvvv` - show log, show and save into file

- high level (multiple nodes access, global changes):
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
        - `-R, --recourse` = `-H, --children`

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

- low level (single node access, local changes): XXX: CRUD?
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



# Problems Solving & Decision Making

- Tree, template parsing recoursing calls how to solve?
    - manage this not in template not allows recoursion, using inline in code instead

- Tree, too slow and recourse in struction, m.b. that's why?:
    - Using Rc, RefCell - no
    - sha256 enabled by default

- Tree links broken
    - tabs and spaces differs

- use bitflags crate?
    - serialize, deserialize needed

- `BTreeMap` vs `Vec<BTreeMap>`
    - no number, random order
    - number order

- Templating
    - `tera` too large resulting executable, removed

- Type field
    - Type field needed, directory and symlink - same and file and symlink - same

- child bitfalgs, and other fields like ignore from parent?
    - or, and, overwrite?

- Remove log:
    - minimize size?
    - no info?

- No interactivity on CLI
    - common practice? current
    - allowing to interact user

- Alias shortcut for some commands (`edit [PATH] -d [DESCRIPTION]` = `desc [DESCRIPTION] [PATH]`)
    - need to make sure what is what file and desc
