use std::error::Error;
use std::fs;

pub struct Requirements {
    pub query: String,
    pub filename: String,
}

impl Requirements {
    // Calling this function "new" is a convention for constructor functions.
    pub fn new(args: &[String]) -> Result<Requirements, &str> {
        if args.len() < 3 {
            return Err(
                "Not enough arguments. Please provide arguments as \"cargo run query filename\"",
            );
        } else if args.len() > 3 {
            return Err(
                "Too many arguments. Please provide arguments as \"cargo run query filename\"",
            );
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            Ok(Requirements { query, filename })
        }
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
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
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
