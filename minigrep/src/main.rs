use std::fs;
use minigrep::config;

fn main() {
    let args = config::parse_config();
    dbg!(args);
}
