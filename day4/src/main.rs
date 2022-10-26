

use std::env;
use std::process;

use day4::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = day4::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}