use minigrep::config;
use std::{fs, process};

fn main() {
    let cfg = config::parse_config().unwrap_or_else(|err| {
        println!("Problem parsing arguments! {}", err);
        process::exit(1);
    });
    dbg!(cfg);
}
