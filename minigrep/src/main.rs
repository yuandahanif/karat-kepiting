use minigrep::{config, file};
use std::process;

fn main() {
    let cfg = config::parse_config().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments! {}", err);
        process::exit(1);
    });

    let contents = file::get_contents(&cfg).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });

    let _ = file::run(&cfg, &contents);
}
