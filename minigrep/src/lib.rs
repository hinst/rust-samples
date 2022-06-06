use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, String> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name.as_str())?;
    println!("{}", file_content);
    return Ok(());
}