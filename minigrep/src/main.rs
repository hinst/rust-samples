use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Unable to read arguments: {}", err);
        process::exit(1);
    });
    println!("Task: search for {} in file {}", config.query, config.file_name);
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Config, String> {
        if args.len() <= 2 {
            return Err(String::from("Not enough arguments. Need two arguments"));
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        let config = Config {
            query,
            file_name,
        };
        return Ok(config);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name.as_str())?;
    println!("{}", file_content);
    return Ok(());
}