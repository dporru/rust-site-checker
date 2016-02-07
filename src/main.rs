extern crate hyper;

use std::io::prelude::*;
use hyper::Client;
use std::fs::File;


fn get_urls(file_name: &str) -> Vec<String> {
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
    let mut new_urls = vec![];
    for url in urls {
        new_urls.push(url.to_string());
    }
    new_urls
}

fn main() {
    let urls = get_urls("urls.txt");
    let client = Client::new();

    for url in urls {
         match client.get(&url).send() {
            Ok(res) => match res.status {
                hyper::Ok => (),
                _ => println!("Error status for '{}': {}", url, res.status),
            },
            Err(_) => println!("Error connecting to '{}'", url)
        };
    }
}
