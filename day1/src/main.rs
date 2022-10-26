
use std::env;
use std::process;

use day1::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = day1::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

/*
use std::fs;
// use std::env;

fn main() {
    let data = fs::read_to_string("src/data.txt").expect("Unable to read file");
    let mut prev: Option<u32> = None;
    let mut curr: Option<u32>;
    let mut cnt: u32 = 0;
    for line in data.lines() {
        curr = Some(line.parse::<u32>().unwrap());
        if let Some(value) = prev {
            if curr.unwrap() > value{
                cnt += 1;
            }
        }
        prev = curr;
    }
    println!("Answer {}", cnt);
}

 */