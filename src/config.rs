use std::io::prelude::*;
use std::fs::File;
use std::path::*;
use toml;

pub struct Config {
    pub raur_root: PathBuf,
}

impl Config {
    pub fn new(config_path: PathBuf) -> Config {
        let mut f = match File::open(config_path) {
            Ok(file) => file,
            Err(e) => panic!(e),
        };
        let mut s = String::new();
        f.read_to_string(&mut s);
        let vals: toml::Value = s.parse().unwrap();
        let path = vals.lookup("paths.root").unwrap();
        Config { raur_root: PathBuf::from(path.as_str().unwrap()) }
    }
}
