/*

fn main() {
    let b = Box::new(5); // Initialization of a box pointer. What is passed inside the parenthesis is what we want to store on the heap.

    // Here, 5 is stored on the heap.
    // On stack, we are storing a pointer to the location of 5.
    // That pointer is stored in b.
    //
    // Boxes have no overhead except storing data in heap.
    // Also no other capabilities.

    println!("b = {}", b);

    // When to use boxes?
    // 1. Have data whose exact size can't be known at compile time and we want to use this value in a context that require the exact size.
    // 2. When we have a large amount of data and want to transfer the ownership of the data but we also want to make sure that the data is not copied because its large amount of data ofcourse.
    // 3. When we own a value and we only care that the value implements a specific trait rather than it being a specific type. This is called trait object and will be discussed later.
}

*/

// Lets look at a case in which boxes are useful.
// We have below, a recursive enum called List.
// Two variants: Cons and Nil.
// Here we cannot know at compile time that what is the length of the enum since, in theory, we can recurse forever here.
// So here, we need to make the code make sense using the box pointer.
// But first, see about Cons-List.
// This is based on the Cons-List data structure in LISP.
// A Cons-List takes in a data, e.g. an integer in a memory cell and the second memory cell points to another Cons-List until it reaches Nil.
// Nil is basically the end of the Cons-List.
// Cons-List data structure is not commonly used in RUST-lang.
// Lets see how boxes can help in our recursive data types.

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// To define a cons-list
use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
}

// The above code doesn't build and gives error "recursive type `List` has infinite size".
// It also tips to insert box and wrap the list into it.
// Lets see how a rust program calculates memory requirement.
// It sees which field of enum takes the more space as only one field will be used at a time.
// The memory space required for the whole enum is equal to the most space the largest variant is going to take up.
//
