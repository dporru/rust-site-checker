extern crate hyper;

use std::io::prelude::*;
use hyper::Client;
use std::fs::File;


struct Sites {
    sites: Vec<String>,
}

impl Sites {
    fn new(file_name: &str) -> Sites {
        let mut f = match File::open(file_name) {
            Ok(f) => f,
            Err(_) => panic!("Couldn't open file '{}'", file_name),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Err(_) => panic!("Couldn't read file '{}'", file_name),
            _ => (),
        };

        let urls: Vec<&str> = s.trim().split("\n").collect();
        let mut sites = vec![];
        for url in urls {
            sites.push(url.to_string());
        }
        Sites {
            sites: sites,
        }
    }
}

fn main() {
    let sites = Sites::new("urls.txt");
    let client = Client::new();

    for site in sites.sites {
         match client.get(&site).send() {
            Ok(res) => match res.status {
                hyper::Ok => (),
                _ => println!("Error status for '{}': {}", site, res.status),
            },
            Err(_) => println!("Error connecting to '{}'", site)
        };
    }
}
