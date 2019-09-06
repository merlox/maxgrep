use std::fs;
use std::error::Error;

pub struct Config {
    pub search: String,
    pub file_name: String,
}

impl Config {
    pub fn new(query: &[String]) -> Result<Config, &'static str> {
        if query.len() < 3 {
            return Err("Insufficient arguments provided make sure to send a search and file_name parameters");
        }
        let search = query[1].clone();
        let file_name = query[2].clone();

        Ok(Config { search, file_name })
    }
}

// The dyn means dynamic which indicates that the Error can be of many different types and we're storing it
// into the heap with the Box<> element
pub fn start(configuration: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(configuration.file_name)?;
    for line in search_case_sensitive(&configuration.search, &file_contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }
    matching_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matching_lines = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matching_lines.push(line);
        }
    }
    matching_lines
}

#[cfg(test)]
mod LibTests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let search_query = "car";
        let contents = "
So there's
this cartoon dealing
with drugs and such
the Cartel.
        ";

        assert_eq!(
            vec!["this cartoon dealing"],
            search_case_sensitive(search_query, contents),
            "The search function didn't find the contents properly"
        );
    }

    #[test]
    fn case_insensitive() {
        let search_query = "CAR";
        let contents = "
So there's
this cartoon dealing
with drugs and such
        ";

        assert_eq!(
            vec!["this cartoon dealing"],
            search_case_insensitive(search_query, contents),
            "The search function didn't find the contents properly"
        );
    }

    // #[test]
    // #[should_panic]
    // fn read_file_should_return_error_with_empty_string_argument() {
    //     read_file("");
    // }
    //
    // #[test]
    // fn read_file_should_return_string() {
    //     let file_contents = read_file("Sup fam")?;
    //     assert_ne!(
    //         file_contents,
    //         (),
    //         "The file contents can't be empty after reading a valid file"
    //     );
    // }
}
