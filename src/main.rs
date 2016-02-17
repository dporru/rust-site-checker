extern crate hyper;
extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::io::prelude::*;
use hyper::Client;
use std::fs::File;
use std::sync::{Mutex, Arc};
use std::thread;


enum SiteStatus {
    None,
    Status(hyper::status::StatusCode),
    ConnectionError,
}


struct Site {
    url: String,
    checked: bool,
    status: SiteStatus,
}

impl Site {
    fn new(url: &str) -> Site {
        Site {
            url: url.to_string(),
            checked: false,
            status: SiteStatus::None,
        }
    }

    fn check(&mut self, client: &Client) {
         self.status = match client.get(&self.url).send() {
            Ok(res) => SiteStatus::Status(res.status),
            Err(_) => SiteStatus::ConnectionError
        }
    }
}

fn get_sites(file_name: &str) -> Vec<Mutex<Site>> {
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
        Mutex::new(Site::new(url))
    }).collect()
}

fn get_cli_args(file_name: &mut String, concurrency: &mut u32, verbose: &mut bool) {
    let file_name_expl = &format!(
        "File containing a list of urls. Default: {}", file_name);
    let concurrency_expl = &format!(
        "Number of simultanious requests, Default: {}", concurrency);
    let verbose_exp = &format!(
        "Verbose output. Default: {}", verbose);
    let mut concurrency_string = "2".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Check server response for a list of urls.");
        ap.refer(file_name)
            .add_option(
                &["-f", "--file-name"], Store, file_name_expl);
        ap.refer(&mut concurrency_string)
            .add_option(
                &["-c", "--concurrency"], Store,
                concurrency_expl);
        ap.refer(verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, verbose_exp);
        ap.parse_args_or_exit();
    }
    *concurrency = concurrency_string.parse().unwrap();
}

fn main() {
    let mut file_name = "urls.txt".to_string();
    let mut concurrency = 2u32;
    let mut verbose = false;
    get_cli_args(&mut file_name, &mut concurrency, &mut verbose);

    let sites = Arc::new(get_sites(&file_name));

    let handles: Vec<_> = (0..concurrency).map(|_| {
        let sites = sites.clone();
        thread::spawn(move || {
            let client = Client::new();
            for site in &mut sites.iter() {
                // Only one thread needs to check a site, so if it's locked
                // it can be skipped.
                let mut site = match site.try_lock() {
                    Ok(site) => site,
                    Err(_) => continue,
                };

                // Only check sites that aren't checked yet.
                if !site.checked {
                    site.check(&client);
                }
            }
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    for site in &sites[..] {
        let site = site.lock().unwrap();

        // Output statuses
        match site.status {
            SiteStatus::None => panic!("'{}' was not checked!", site.url),
            SiteStatus::ConnectionError =>
                println!("Error conneting to '{}'", site.url),
                SiteStatus::Status(status) => match status {
                    hyper::Ok if !verbose => (),
                    hyper::Ok if verbose => println!(
                        "Success status for '{}': {}", site.url, status),
                    _ => println!("Error status for '{}': {}", site.url, status),
                },
        };
    }
}
