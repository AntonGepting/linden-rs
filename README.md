# Linden

## Description

Linden is a program for building file trees for projects and managing meta
information (descriptions, comments, tags) written using Rust programming language.


## Motivation

Sometimes there is a need to add to project files some description for better
understanding of structure and functionality of them. Not all file systems
"out-of-the-box" support such a feature. Anyway descriptions need to be
"transported" with the project itself.


## Target

Aiming targets:
- Storing and editing meta information for project files:
    - read (`ls`, `print`, ...)
    - write (`edit`, `sort`, ...)
    - export (`stdout`, `txt`, `md`, `html`)
- Human readable/writable database file:
    - YAML file (current choice)

## Usage



## Dependencies

- [clap](https://crates.io/crates/clap) (2.33.3) - for CLI (Apache License Version 2.0 January 2004)
- [log](https://crates.io/crates/log) (0.4.11) - for logging (Apache License Version 2.0 January 2004)
- [log4rs](https://crates.io/crates/log4rs) (0.13.0) - for logging targets (Apache License Version 2.0 January 2004)
- [dirs](https://crates.io/crates/dirs) (2.0.1) - for using `XDG_HOME` environment variable (Apache License Version 2.0 January 2004)
- [chrono](https://crates.io/crates/dirs) (0.4.19) - for time storing and convertion (dual-licensed MIT License and Apache 2.0 License)
- [serde](https://crates.io/crates/serde) (1.0.116) - for serializing/deserializing (Apache License Version 2.0 January 2004)
- [serde_derive](https://crates.io/crates/serde_derive) (1.0.116) - for serializing/deserializing (Apache License Version 2.0 January 2004)
- [serde_yaml](https://crates.io/crates/serde_yaml) (0.8.13) - for YAML format support (Apache License Version 2.0 January 2004)


## License

Linden is licensed under the MIT license. Please read the license file in the repository for more information.
