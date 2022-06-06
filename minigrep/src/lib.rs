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

fn search(content: &str, query: &str) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(String::from(line));
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_one() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(content, query));
    }
}