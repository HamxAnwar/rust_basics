// We can create documentation of our code.
// or /* is used for comments. If we put ///, it would be used for documentation generation.

/// Favourite color.
enum Color {
    Red,
    Blue,
}

/// A piece of mail.
struct Mail {
    address: String,
}

/// Ads two numbers.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world!");
}

// Run the above using cargo doc which generated the documentation in "/home/loki/RustroverProjects/documentation/target/doc/documentation/index.html".
