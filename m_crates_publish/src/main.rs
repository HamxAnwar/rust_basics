// Also open and refer to the crate called art. This is the explaination of what is happening there.
// More specifically "Re-exporting".
// Two modules:
//  1. Kinds
//      1. PrimaryColor enum
//      2. SecondaryColor enum
//  2. Utils
//      Contains a function called utils which take two primary color and mi them to return a secondary color.
// The structure might not make sense to the people that are or will use our library.
// We do cargo doc to generate the documentation.
// Also if we do the cargo doc --open and want to move to the PrimaryColor, we can see that we can do that by going to the kinds module first.
// To make sure that our enums are open to all, we can re-export the enums using the pub use keyword as in the lib.rs
//      pub use self::kinds::PrimaryColor;
//      pub use self::kinds::SecondaryColor;
//      pub use self::utils::mix;
// When the doc is regenerated, we can see the re-export on the main page of the documentation.
// We can also see that in the main file, we do not need to import the enum and function through the full path. Rather, we can do that from the top level of our library.
//
// To publish your crate on crates.io, sign up on it using github.
// Profile -> Account settings -> API tokens -> New token.
// On the command line -> cargo login.
// Now we can publish the crate, but we need to check our meta data.
// Name is important to be published.
// Also remember to verify the email at crates.io.
// Also remember to add the description and liscence to the toml file.
// Now cargo publish to publish the crate to crates.io using "cargo publish" command.
// Remember that publishing to crates.io is permanent and we cannot delete what has been published.
// This helps to not break other's code if they use our crate.
// We can upload a different version of our crate (modified one) by changing the version in the toml file publishing again.
// Though we can't delete or modify the crate but we can stop the downloading of a specific version of our crate using the yank command.
// So we do "cargo yank --vers VersionNumber" to yank our crate.
// This will let the already users to use this crate but new users can't use it.
// To undo the yank -> "cargo yank --vers VersionNumber --undo".

fn main() {
    println!("Hello, world!");
}
