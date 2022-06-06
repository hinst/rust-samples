use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_arguments(&args);
    println!("Search for {} in file {}", config.query, config.file_name);
    let file_content = fs::read_to_string(config.file_name.as_str())
        .expect(format!("Read file {}", config.file_name).as_str());
    println!("{}", file_content);
}

struct Config {
    query: String,
    file_name: String,
}

fn parse_arguments(args: &[String]) -> Config {
    let query = String::from(&args[1]);
    let file_name = String::from(&args[2]);
    return Config {
        query,
        file_name,
    };
}
