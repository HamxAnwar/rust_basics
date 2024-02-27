// Derive is a special macro to put some functionality in enums and structs.
/* we can make use of it with

#[derive(Debug)]

to debug printing functionality.

*/

// Another derive functionalities that can be used are:
// - Clone
// - Copy

// It let the compiler automatically creates a copy of the data when storing into a struct or a function.
// Ownership is not transferred automatically and makes a copy instead this way.
// Try to use clone/copy to a struct which stores fewer data such as four or five entries. Cloning and copying more entry structs could result in making highly expensive copies of the data without realising it.

fn main() {
    println!("Hello, world!");
}