use std::env;
use std::process;

use advent_of_code::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Solving day: {}", config.day);
    println!("Taking input from {}", config.file_path);

    if let Err(e) = advent_of_code::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
