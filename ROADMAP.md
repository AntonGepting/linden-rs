# Roadmap


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
