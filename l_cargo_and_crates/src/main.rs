// Cargo is the rust packaging system.
// Profiles define how the code is to be compiled in cargo.
// Cargo has two profiles:
//  - Dev: Good defaults for Development build. (Unoptimized)
//  - Release: Release defined the release build with good defaults. (Optimized)
//  Simply doing cargo build will compile the code in the dev profile as shown with the finished
//  line in cmd.
//  cargo build --release will compile the code in release profile.
//  Optimization level for both profiles can be specified in cargo.toml. Go to file for further
//  information.
//  Lets get a crate ready for publishing as we have previously used other's codes. First we will
//  see about documentation comments which are given by //! or ///.

/// Another doc comment is given by //!.
/// It wont document the information following the comment but the information inside the comment.
/// This is the difference between //! and ///.
//! #My Crate
//!
//! 'my_crate' is a collection of utilities to make performing certain calculations more convenient.

/// Adds one to the number give.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg)
///
/// assert_eq!(6, answer);
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Now to build documentation comments for our crate, we use "cargo doc --open".
// Now apart from the examples, there are few other sections:
//      - Panics section: Describes scenarios in which a function would panic.
//      - Errors section: If the function has a return type as Result, it is common to have an
//      errors section. Describes what kind of error can happen and what conditions have to occur
//      for those errors to occur.
//      - Safety section: If the function is unsafe, describes why is it unsafe and under what
//      conditions would it effect.
// Also rememeber, running cargo test will execute the example in the doc comments.
