// Dref trait allows us to use pointers as normal references.
/*
fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // dref of y
                       // assert_eq!(5, y);
    assert_eq!(5, *z);
}
*/
// In the code above, the dereferencing operator '*' used with a pointer dereferences the memory location and returns the value.
// We can see that if we don't the *, it gives an error that says can't compare an integer with a reference to the integer.
// Also we can see that we can use z as above using the box pointer which will result in the same result.
// Z is pointing to a copy of x or 5 because remember when we input a static type to a function, the value is copied.

// Lets understand this dref by making a box smart pointer by our own.

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // Associated types are a different way to defining generic types.

    fn deref(&self) -> &T {
        &self.0
    }
}

// One difference between the box pointer and our box pointer is, x is not stored on the heap.
/*
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/
// DO rememeber that the deref drefs the trait outputs a reference.
// This reference can dereferenced normally by rust.
// So line 46 will be like:
//          assert_eq!(5, *(y.deref()));
//
// There is also implicit deref coercion that happens automatically for deref trait. It happens when we need to convert one reference to another reference.

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let name = MyBox::new(String::from("Hamza"));
    hello(&name);
}

// Here if there was no deref coercion, we would have written it like:
//             hello(&(*name)[..])
// &MyBox<String> -> &String -> &str
// Rust performs the above conversion automatically.
// Rust will perform deref coercion in the following conditions:
// - From immutable reference to another immutable reference.
// - From mutable reference to another mutable reference.
// - From mutable reference to immutable reference.
// Rust will not do coercion when going:
// - From immutable reference to mutable reference.
// This is due to borrowing rules.
