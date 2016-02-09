# Rust SiteChecker

This is a very simple site status checker written in Rust.

## Usage

Add the urls you want to check to `urls.txt` and run the program (e.g. `cargo
run` or the binary if you've build it already). Make sure the `urls.txt` is in
your working directory. You can also give a path to another file containing the
urls using the `-f` flag.

```
$ site_checker -f /path/to/urls.txt
```

The program only gives output for responses that don't have a `200` status.
