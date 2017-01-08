use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::*;
use toml;

pub struct Config {
    pub raur_root: PathBuf,
}

impl Config {
    pub fn new(config_path: &PathBuf) -> Config {
        let mut f = match File::open(config_path) {
            Ok(file) => file,
            Err(_) => generate_config(config_path),
        };
        let mut s = String::new();
        f.read_to_string(&mut s);
        let vals: toml::Value = s.parse().unwrap();
        let path = vals.lookup("paths.root").unwrap();
        Config { raur_root: PathBuf::from(path.as_str().unwrap()) }
    }
}

fn generate_config(path: &PathBuf) -> File {
    println!("No config found!\nEnter desired raur root dir> ");
    let mut f = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!(e),
    };
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    write!(f, "[paths]\nroot = \"{}\"\n", input.trim());
    f
}