extern crate hyper;

use hyper::client;
use std::io::Read;

pub fn search_by_name(name: &str) -> String {
    let url = "/rpc/?v=5&type=search&by=";
    let field = "name";
    let query = format!("https://aur.archlinux.org{}{}&arg={}", url, field, name);
    let client = client::Client::new();
    let mut res = client.get(&query).send().unwrap();
    let mut buf = String::new();
    res.read_to_string(&mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name_works() {
        use hyper::status;
        assert_eq!(search_by_name("cower"), status::StatusCode::Ok);
    }
}
