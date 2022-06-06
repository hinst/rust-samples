use std::env;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Unable to read arguments: {}", err);
        process::exit(1);
    });
    println!("Task: search for {} in file {}", config.query, config.file_name);
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
