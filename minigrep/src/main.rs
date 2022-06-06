use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("Search for {} in file {}", query, file_name);
    let file_content = fs::read_to_string(file_name).expect(format!("Read file {}", file_name).as_str());
    println!("{}", file_content);
}
