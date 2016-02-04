# Rust SiteChecker

This is a very simple site status checker written in Rust.

## Usage

Add the urls you want to check to `urls.txt` and run the program (e.g. `cargo
run` or the binary if you've build it already). Make sure the `urls.txt` is in
your working directory. (I did say it was simple ;).

The program only gives output for responses that don't have a `200` status.
