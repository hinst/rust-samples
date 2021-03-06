use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn from_args(mut args: env::Args) -> Result<Config, String> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err(String::from("Query string is missing from the argument list")),
        };
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err(String::from("File name is missing from the argument list")),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let config = Config {
            query,
            file_name,
            case_sensitive,
        };
        return Ok(config);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name.as_str())?;
    let results = if config.case_sensitive {
        search(file_content.as_str(), config.query.as_str())
    } else {
        search_case_insensitive(file_content.as_str(), config.query.as_str())
    };
    for line in results {
        println!("{}", line);
    }
    return Ok(());
}

fn search(content: &str, query: &str) -> Vec<String> {
    return content.lines()
        .filter(|line| line.contains(query))
        .map(|line| String::from(line))
        .collect();
}

fn search_case_insensitive(content: &str, query: &str) -> Vec<String> {
    let query = query.to_lowercase();
    let mut results: Vec<String> = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(query.as_str()) {
            results.push(String::from(line));
        }
    }
    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(content, query));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(content, query)
        );
    }
}
