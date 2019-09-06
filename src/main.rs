use std::env;
use std::process;
use maxgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration = Config::new(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    println!("Searching '{}' in the '{}' file:", configuration.search, configuration.file_name);

    if let Err(e) = maxgrep::start(configuration) {
        process::exit(1);
    }
}
