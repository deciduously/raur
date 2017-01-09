extern crate hyper;
// extern crate serde_json;

use hyper::client;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::*;
use std::process::Command;

// pub struct PackageInfo {
// name: &'static str,
// version: &'static str,
// description: &'static str,
// maintainer: &'static str,
// votes: u32,
// }
//
// impl PackageInfo {
// pub fn new(name: &'static str,
// version: &'static str,
// description: &'static str,
// maintainer: &'static str,
// votes: u32)
// -> PackageInfo {
// PackageInfo {
// name: name,
// version: version,
// description: description,
// maintainer: maintainer,
// votes: votes,
// }
// }
// }

// downloads given package to given destination, returns a Result with the path of the file written
pub fn download_package(name: &str, dest: PathBuf) -> Result<PathBuf, &'static str> {
    let url = "https://aur.archlinux.org/cgit/aur.git/snapshot/";
    let file_str = format!("{}.tar.gz", name);
    let mut file_path = dest;
    file_path.push(&file_str);
    let display = file_path.display();
    let mut file = match File::create(&file_path) {
        Err(reason) => {
            println!("could not create {}: {}", display, reason.description());
            return Err("could not create file");
        }
        Ok(file) => file,
    };

    let query = format!("{}{}", url, &file_str);
    let client = client::Client::new();

    let mut res = client.get(&query).send().unwrap();
    let mut bytes: Vec<u8> = Vec::new();
    let len = res.read_to_end(&mut bytes).unwrap();

    match file.write_all(&bytes) {
        Err(reason) => {
            println!("could not create {}: {}", display, reason.description());
            Err("could not create file")
        }
        Ok(_) => {
            println!("successfully wrote to {}", display);
            Ok(PathBuf::from(file_path.as_os_str()))
        }
    }
}

pub fn install_package(package: &str, loc: PathBuf) -> Result<PathBuf, &'static str> {
    let path = match download_package(package, PathBuf::from(loc.as_os_str())) {
        Ok(path) => path,
        Err(e) => panic!(e),
    };
    match untar_package(path.to_path_buf()) {
        Ok(_) => Ok(PathBuf::from(loc.as_os_str())),
        Err(_) => Err("could not untar"),
    }
}

// pub fn search_by_name(name: &str) {
// let url = "/rpc/?v=5&type=search&by=";
// let field = "name";
// let query = format!("https://aur.archlinux.org{}{}&arg={}", url, field, name);
// let client = client::Client::new();
// let mut res = client.get(&query).send().unwrap();
//
// let mut buf = String::new();
// res.read_to_string(&mut buf);
//
// let result: serde_json::de::Deserialize = serde_json::from_str(&buf).unwrap();
// println!("{}", result);
//
// }

fn untar_package(archive: PathBuf) -> Result<PathBuf, &'static str> {
    let untar = Command::new("tar")
        .current_dir(archive.parent().unwrap())
        .arg("xvf")
        .arg(archive.file_name().unwrap())
        .output()
        .expect("could not untar package archive");
    println!("{}", untar.status);

    Ok(archive)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_url_works() {
        // assert_eq!(get_url("cower", "download"), "https://aur.archlinux.org/cgit/aur.git/snapshot/cower.tar.gz");
    }
}
