Binary crate: Executable crate


- In the cargo.toml, we can define crates.
- Initially, we have a single crate even if no crate is defined in the cargo.toml.
  - If there is a main.rs in src directory, then a binary crate with the same name as the package is already created and main.rs will be the crate's root.
  - If there is a lib.rs in src directory, then a library crate with the same name as the package is already created and lib.rs will be the crate's root.
- A package must have one crate.
- A package could have 0 or 1 library crate.
- A package could have many numbers of binary crate.
- One binary crate which is the root is in src directory. Other binary crates should be in src/bin folder.
