//extern crate dirs;
extern crate serde_yaml;

use crate::error::Error;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const CFG_DEFAULT_DIR: &str = "linden";
pub const CFG_DEFAULT_FILENAME: &str = "linden.yml";
// XXX: $XDG_CONFIG_HOME
// XXX: $XDG_DATA_DIRS
pub const CFG_SYSTEM_PATH: &str = "/usr/share/linden";

pub const LOG_CONFIG_OPEN_CONFIG_FILE: &str = "Config: loading config file";
pub const LOG_CONFIG_DEFAULT_CONFIG_FILE: &str = "Config: init runtime default config";
pub const LOG_CONFIG_OPEN_CUSTOM_CONFIG_FILE: &str = "Config: open custom config file";
pub const LOG_CONFIG_CUSTOM_CONFIG_FILE_NOT_EXISTS: &str = "Config: custom config file not exists";

pub const LOG_CONFIG_SAVE: &str = "Config: save";
pub const LOG_CONFIG_READ: &str = "Config: read";
pub const LOG_CONFIG_WRITE: &str = "Config: write";

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<bool>,
    // XXX: type may be important (dir, file, symlink), tags too
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_desc: Option<BTreeMap<String, String>>,
}

impl Config {
    pub fn new() -> Self {
        Default::default()
    }

    //pub fn default_desc() {
    //let mut bt = BTreeMap::new();
    //bt.insert("Cargo.toml", "Cargo configuration file");
    //unimplemented!();
    //}

    fn get_home() -> PathBuf {
        let mut cfg_home_path = dirs::config_dir().unwrap();
        cfg_home_path.push(CFG_DEFAULT_DIR);
        cfg_home_path.push(CFG_DEFAULT_FILENAME);
        cfg_home_path
    }

    // checking config name
    //  if specified use given path
    //  else use home
    //  else use system
    //  else create default in memory
    //
    // TODO: not exists, refactor!!!
    //
    pub fn load(filename: Option<&PathBuf>) -> Result<Config, Error> {
        let cfg_home_path: PathBuf;
        let cfg_system_path: PathBuf;
        // if custom cfg exists
        if let Some(cfg_custom_path) = filename {
            if Path::new(&cfg_custom_path).exists() {
                info!("{}", LOG_CONFIG_OPEN_CUSTOM_CONFIG_FILE);
                Ok(Config::read(&cfg_custom_path)?)
            } else {
                Err(Error::new(
                    LOG_CONFIG_CUSTOM_CONFIG_FILE_NOT_EXISTS.to_string(),
                ))
            }
        } else {
            let mut paths = Vec::new();
            // generate home path, and push into variants
            cfg_home_path = Config::get_home();
            paths.push(&cfg_home_path);
            // generate system path, and push into variants
            // NOTE: OS dependency
            cfg_system_path =
                PathBuf::from(format!("{}/{}", CFG_SYSTEM_PATH, CFG_DEFAULT_FILENAME));
            paths.push(&cfg_system_path);
            // check list of path if any exists, open and return
            for path in paths {
                if Path::new(&path).exists() {
                    info!("{}: {:?}", LOG_CONFIG_OPEN_CONFIG_FILE, path.to_str());
                    return Ok(Config::read(&path)?);
                }
            }
            info!("{}", LOG_CONFIG_DEFAULT_CONFIG_FILE);
            Ok(Config::new())
        }
    }

    #[allow(dead_code)]
    pub fn save(&self, filename: Option<&Path>) -> Result<(), Error> {
        let path = match filename {
            Some(s) => s.to_path_buf(),
            None => Config::get_home(),
        };
        info!("{}: {}", LOG_CONFIG_SAVE, &path.to_str().unwrap());
        self.write(&path)?;
        Ok(())
    }

    // write config structure to file
    #[allow(dead_code)]
    pub fn write(&self, filename: &Path) -> Result<(), Error> {
        info!("{}: {}", LOG_CONFIG_WRITE, &filename.to_str().unwrap());
        let config_str = serde_yaml::to_string(&self)?;
        fs::write(filename, config_str.as_bytes())?;
        Ok(())
    }

    // read config structure from file
    pub fn read(filename: &Path) -> Result<Self, Error> {
        info!("{}: {}", LOG_CONFIG_READ, filename.to_str().unwrap());
        let content = fs::read_to_string(filename)?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}
