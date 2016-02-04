extern crate hyper;

use hyper::Client;


fn main() {
    let sites = vec![
        "http://httpstat.us/200",
        "http://httpstat.us/500",
        "http://httpstat.us/502",
    ];

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
