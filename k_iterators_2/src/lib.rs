use std::error::Error;
use std::{env, fs};

pub struct Requirements {
    pub query: String,
    pub filename: String,
}

impl Requirements {
    // Calling this function "new" is a convention for constructor functions.
    pub fn new(mut args: env::Args) -> Result<Requirements, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string."),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a filename."),
        };

        Ok(Requirements { query, filename })
    }
}

// This is the running function.
pub fn run(requirements: Requirements) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(requirements.filename)?;

    for line in search(&requirements.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

// We need to specify the lifetime of return below to the content string as the query lies inside the content.

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    query.to_lowercase();
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(query))
        .collect()
}

// Test - Failing test

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}
