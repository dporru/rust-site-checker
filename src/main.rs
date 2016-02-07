extern crate hyper;
extern crate argparse;

use argparse::{ArgumentParser, Store};
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
    urls.iter().map(|url| {
        url.to_string()
    }).collect()
}

fn get_cli_args(file_name: &mut String) {
    let file_name_expl = &format!(
        "File containing a list of urls. Default: {}", file_name);
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Check server response for a list of urls.");
        ap.refer(file_name)
            .add_option(
                &["-f", "--file-name"], Store, file_name_expl);
        ap.parse_args_or_exit();
    }
}

fn main() {
    let mut file_name = "urls.txt".to_string();
    get_cli_args(&mut file_name);
    let urls = get_urls(&file_name);
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
