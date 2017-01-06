#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
extern crate raur;

fn main() {
    let matches = clap_app!(raur =>
        (version: "0.1.0")
        (author: "deciduously <bendlovy@gmail.com>")
        (about: "Simple CLI AUR helper")
        (@arg DOWNLOAD: -d --download +takes_value "Downloads the selected package to your build folder")
        (@arg SEARCH: -s --search +takes_value "Searches AUR for given package and returns info")
    ).get_matches();
        
    println!("doing thing on {}", matches.value_of("DOWNLOAD").unwrap());
}
