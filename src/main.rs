extern crate hyper;

use std::io::prelude::*;
use hyper::Client;
use std::fs::File;


fn main() {
    let mut f = File::open("urls.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let sites: Vec<&str> = s.trim().split("\n").collect();
    let client = Client::new();

    for site in sites {
         match client.get(site).send() {
            Ok(res) => match res.status {
                hyper::Ok => (),
                _ => println!("Error status for '{}': {}", site, res.status),
            },
            Err(_) => println!("Error connecting to '{}'", site)
        };
    }
}
