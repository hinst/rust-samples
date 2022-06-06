use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Unable to read arguments: {}", err);
        process::exit(1);
    });
    println!("Search for {} in file {}", config.query, config.file_name);
    let file_content = fs::read_to_string(config.file_name.as_str())
        .expect(format!("Read file {}", config.file_name).as_str());
    println!("{}", file_content);
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn from_args(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(String::from("Not enough arguments"));
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