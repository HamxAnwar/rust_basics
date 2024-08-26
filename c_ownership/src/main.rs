/*
Every program must track memory.
If they fail to do so, a leak will occur.
Every language has its own method of doing it. Rust uses ownership.
It uses the concept of stack and heap.
During runtime, program has both stack and heap.
Stack: fixed size.
    Stores stack frames which are made for every executed function.
    Stack frames store local variables for those functions.
    Size calculated at compile time.
Heap: Dynamic size.
    Less organized.
    Can grow or shrink on runtime.
    Can store large, dynamic sized data.
    We controls the lifetime of the data.
Pushing to stack is faster than allocating memory on the heap as then we have to go through a pointer stored in the stack, pointing to the data in heap.
For example: String datatypes are stored on heap as they are dynamically allocated while str are stored in stack.
The owner is simply a function in rust.
The memory can be either moved or borrowed from the owner.
There are three ownership rules, we need to remember:
    1. Each value in Rust has a variable that's called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.

When we define:
            let x = 5;
            let y = x;              // x is saved on stack and this will copy the value of x to y.

            let s1 = String::from("hello");
            let s2 = s1;           // Here the s1 will move to s2 and s1 will be dropped.
    If we want to copy the value from the heap, we can use the clone method or the copy trait.
            let s1 = String::from("hello");
            let s2 = s1.clone();
    Cloning is more expensive than copying.

Similarly when we define a function and pass a variable to it, it is same as assigning it to another variable.
-----------------------------------
fn main() {
    let s = String::from("abc");
    takes_ownership(s);
    println!("{s}");
}

fn takes_ownership(some_string) {
    println!("{}", some_string);
}
-----------------------------------

In the above section, the main function would not print 's' in the last println statement as when we call the take_ownership function, the ownership of 's' is given to the function and as the function completes, it drops the variable 's'.
To return ownership from a function, we need to return the variable from the function.
Now instead of returning everytime a variable from a function, we can use references such as:
    fn take_ownership(s: &String) {}

References are immutable by default.
We cannot modify the value and to modify it, we can use mutable references.
    fn take_ownership(s: &mut String) {}

Note:
    We can only have one mutable reference in a particular scope to a particular data.
    We cannot have a mutable reference if immutable references are already defined.
    The scope of a immutable reference starts at its definition and ends at the last use of it.

        fn main() {
            let r1 = &s;                    // Scope starts
            let r2 = &s;                    // Scope starts

            println!("{}, {}", r1, r2);     // Scope ends

            let r3 = &mut s;                // Out of the scope of the immutable references.
        }
*/

// --------------------------------

// #[derive(Clone, Copy)]
/*enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("Bright!"),
        Light::Dull => println!("Dull!")
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(dull);
    display_light(dull);
}*/

// The above code gives error as the use of display_light function two times throws an ownership error.
// The dull variable ownership moves from main to the display_light function.
// Upon successful execution, the display_light function deletes the dull variable.
// Calling it a second time through error.

// ------------------------------------------------------------------

// We can tackle the above by borrowing as below:

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright!"),
        Light::Dull => println!("Dull!"),
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}
