// lets make a minigrep tool in RUST. This is the first real project, I am looking forward to...
// The tool will be able to search for a string within a file.
// This program works as case insensitive.

use std::{env, process};
use CLI_tool::{run, Requirements};

fn main() {
    // We want the user to pass in a string and a file name.

    let args: Vec<String> = env::args().collect();

    // In the above, args will let us take the input parameters while collect lets us collect the input arguments into the vector.
    // We also want to specify the type of arguments to be collected as the collect function needs to know it.

    // eprintln!("args = {:#?}", args);

    // The eprintln is the standard printline statement for the error. So if the program is being run as cargo run > output.txt, in case of simple println statement, the output.txt file will also contain error it throws out. Using eprintln wont through out errors but just positive results in the output.txt.

    let arguments = Requirements::new(&args).unwrap_or_else(|err| {
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
