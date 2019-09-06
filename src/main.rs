use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::new(&args);

    println!("Searching {} in the {} file", configuration.search, configuration.file_name);

    let file_contents = fs::read_to_string(configuration.file_name).expect("Couldn't read the file contents");
}

struct Config {
    search: String,
    file_name: String,
}

impl Config {
    fn new(query: &[String]) -> Config {
        let search = query[1].clone();
        let file_name = query[2].clone();

        Config { search, file_name }
    }
}
