use std::env;
use std::process;

use minigrep::*;

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Unable to read arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
