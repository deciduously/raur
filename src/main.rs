#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate raur;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let keywords = &args[1]; //package to search for

    println!("{}", raur::search_by_name(keywords));
}
