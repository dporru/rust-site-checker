# Rust Site Checker

This is a simple site status checker written in Rust.

## Installation

```bash
$ cargo install site_checker
```

## Usage

Add the urls you want to check to `urls.txt` and run the program:
```bash
$ site_checker -f /path/to/urls.txt
```

When the `-f` flag is ommitted site_cecker will try to open `urls.txt` in the
current directory.

By default the site_checker command only gives output for responses that don't
have a `200` status. With the `-v` flag statuses for all responses are shown.

By default two threads are used to check urls. To change this behavior use to
`-c` flag:

```bash
$ site_checker -c 4 -f /path/to/urls.txt
```

Will set a concurrency of 4.
