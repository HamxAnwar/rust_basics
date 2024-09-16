// lets make a minigrep tool in RUST. This is the first real project, I am looking forward to...
// The tool will be able to search for a string within a file.
// This program works as case insensitive.

use k_iterators_2::{run, Requirements};
use std::{env, process};

fn main() {
    let arguments = Requirements::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for the query: {} in the file: {}",
        arguments.query, arguments.filename
    );
    // eprintln!("query = {:#?}", query);
    // eprintln!("filename = {:#?}", filename);

    if let Err(e) = run(arguments) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
