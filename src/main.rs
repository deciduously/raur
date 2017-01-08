#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate raur;
extern crate toml;

mod config;

use std::env;
use std::path::*;

fn main() {
    let matches = clap_app!(raur =>
        (version: "0.1.0")
        (author: "deciduously <bendlovy@gmail.com>")
        (about: "Simple CLI AUR helper")
        (@arg CONFIG: -c --config +takes_value "Specifies a config to use other than the default")
        (@arg DOWNLOAD: -d --download +takes_value "Downloads the selected package to your build folder")
        (@arg SEARCH: -s --search +takes_value "Searches AUR for given package and returns info")
    ).get_matches();

    // this probably needs to qualify the full path
    let config_path = if matches.is_present("CONFIG") {
        PathBuf::from(matches.value_of("CONFIG").unwrap())
    } else {
        get_default_config()
    };

    let config = config::Config::new(&config_path);

    let package = matches.value_of("DOWNLOAD").unwrap();
    println!("{}",
             raur::download_package(package, config.raur_root).unwrap());
}

fn get_default_config() -> std::path::PathBuf {
    let mut ret = env::home_dir().unwrap();
    ret.push(".config");
    ret.push("raur");
    ret.push("raur");
    ret.set_extension("toml");
    ret
}
