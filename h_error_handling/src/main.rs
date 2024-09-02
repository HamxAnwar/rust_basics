//use std::fs::File;
//fn main() {
//    //panic!("Program crashed");      // If we cannot deal with the error gracefully, we can use the
//                                    // panic macro which will panic the program and crash it with
//                                    // the given error message.
//                                    // We can also use "RUST_BACKTRACE=1 cargo run" to backtrace
//                                    // the issue and get a better view of the originating problem.
//
//    // Recoverable error handling!
//    // Result enum which is given as:
//    /*
//    enum Result {
//        Ok(T),
//        Err(E),
//    }
//    */
//
//    let f = File::open("hello.txt");
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => panic!("Problem opening the file with error: {:?}", error),
//    };
//}





//use std::fs::File;
//use std::io::ErrorKind;
//
//fn main() {
//    // For handling the above error and code more efficiently, we can do the following:
//
//    let f = File::open("hello.txt");
//    let f = match f {
//        Ok(file) => file,
//        Err(error) => match error.kind() {
//            ErrorKind::NotFound => match File::create("hello.txt") {
//                Ok(fc) => fc,
//                Err(e) => panic!("Problem creating the file: {}", e),
//            },
//            other_error => {
//                panic!("Problem opening the file: {}", other_error);
//            }
//        }
//
//    };
//}





// Handling errors like above using nested match statements, makes the code hard to write and read.
// We can use another way to do it is using enclosures as follows:

use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // For handling the above error and code more efficiently, we can do the following:

    let f = File::open("hellow.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hellow.txt").unwrap_or_else(|error|{
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // We have some other useful functions related to the result enum which we will discuss as follows:
    // We can use the unwrap instead of match expression.

    let f = File::open("hello.txt").unwrap();

    // We can also provide a custom error message with the expect function.

    let f = File::open("hello.txt").expect("This is the desired error message!");
    
    use std::io::{self, Read};
    use std::fs::File;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("Hello.txt")?;       // The ? operator or a result enum is similar to
                                                // the unwrap_or_expect method. It returns either 
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)

        // Now instead of above, we can further simplify it by chaining the calls as below:
 
        let mut s = String::new();
        File::open("Hello.txt")?.read_to_string(&mut s)?;                  // The ? operator or a result enum is similar to
                                                                           // the unwrap_or_expect method. It returns either
                                                                           // a result, in this case a file, or an error
                                                                           // instead of panicing.
        Ok(s)
    }
}

// Since the main function doesn't return anything, we cannot use the ? operator in it. If we
// still want to, we can but we have to change the return of the main. Main function is
// special so it can only return certain item types. for example.

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {       // The box is a trait object which here means that the
                                                // main function returns a unit which means
                                                // nothing, shown by the parenthesis or any type of
                                                // error.
   let f = File::open("hello.txt")?;
   Ok(())
}

// But when do we use the panic macro and the result enum.
// We need to use, by default, the result.
// Panic should be only used in an example code or in cases where the error cannot be handled and
// the program needs to be closed when trying to recover.
// Unwrap and expect can be used in development code or example code and in production code, they
// need to be replaced with proper error handling.
// We can also use expect and unwrap when we know that a call will succeed.
