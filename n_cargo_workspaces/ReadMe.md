- Hey guys!
- What if our projects extend too much. We have multiple library crates.
- We can use workspaces to manage such projects.
- Workspaces help us manage multiple related packages that are developed at random.
- Packages in a single workspace share multiple dependencies by having one cargo.lock file.
- These packages also share one output directory and various settings too.
- This is the workspace formation.
- We will create one workspace with two libraries.

  - One will have an add one function.
  - Other will have an add two function.

- Make a directory with the workspace name.
- Make a .toml file in it with the workspace tag instead of packages/dependencies.
- Packages in a workspace are called workspace members.
- Enter the members and make the packages with cargo new command.
- One will be cargo new adder and other will be cargo new add-one --lib (we need to make it a library).
- Modify them and then run cargo build to build them.
- Even if we run cargo build from the packages, we will see the lock file in the top level i.e. the workspace level.
- Cargo does this because all the packages in a workspace need to be dependent on each other.
- We set a function to add one inside the add-one library.
- Call this function in the other package.
- To do the above, we need to add this add-one as a dependency to our adder binary in the Cargo.toml file.
- We have explicitly define the path of the library.
- Remember, when the library is called in the adder main file and we use it there, we include by add_one and not add-one.
- After completing the code, we need to build the root of the package again and then run:
  cargo run -p adder --> This will tell which package to run.
- Moreover, if we want external dependencies, we can see that both packages have toml files but there is only one lock file in the top workspace.
- To make the dependency available for both packages, we have to put the dependency in both toml files but only one will be downloaded.
- This makes sure that all packages in a workspace use same version of a dependency crate.

- If we add a test package inside our add-one package. Run cargo test, it will show that the test ran for which package and what are the results.
- Running cargo test in the root of our workspace will run test in all the packages in our workspace.
- To run test for specific project, we can do "cargo test -p package_name".
- Lastly, if we want to publish packages within a workspace, we need to do it individually for every package by cd into the package and then run cargo publish.
