extern crate hyper;

use std::io::prelude::*;
use hyper::Client;
use std::fs::File;


fn main() {
    let mut f = match File::open("urls.txt") {
        Ok(f) => f,
        Err(_) => panic!("Couldn't open file 'urls.txt'"),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(_) => panic!("Couldn't read file 'urls.txt'"),
        _ => (),
    };

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
