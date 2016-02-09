# Rust SiteChecker

This is a very simple site status checker written in Rust.

## Installation

```bash
$ cargo install site_checker
```

## Usage

Add the urls you want to check to `urls.txt` and run the program:
```
$ site_checker -f /path/to/urls.txt
```

The program only gives output for responses that don't have a `200` status.
