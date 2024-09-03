// Dingling references...A reference that point to invalid data.

/*
fn main() {
    let r: &i32; // Initialized variable.

    {
        let x = 5;
        r = &x;
    }

    eprintln!("r = {:#?}", r);
}
*/

// The above code has a dingling reference since r is set as &x where we want to print r when the scope of x is ended. So it points a invalid reference.
// Rust knows it at compile time. It calculates it during compile time using borrow checker.
// The borrow checker runs at compile time and estimates if all the borrowed values are valid or not.
// But what if the borrow checker is not sure about the lifetime of a variable or a return from a function. We need to help the borrow checker to estimate that what is the lifetime of a variable outside of its scope as above? It can be achieved using lifetimes.
// Have a look at the code below:

/*
fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    eprintln!("result = {:#?}", result);
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// In the above code, we can see that the function longest returns a reference to the longest string. x and y, in this case, have same lifetime but in other cases, could have different lifetimes.
// We also dont know what is the lifetime of the returned result which can be seen when we uncomment the above code.
// To fix this, we need to use "Generic Lifetime Annotation" or simply called lifetimes.
// It doesn't change the lifetime of a reference but only explains the relation between lifetimes of various references.
// We can implement it as the following:

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    eprintln!("result = {:#?}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {     // We can name a lifetime anything we want but the convention is to start it with ' and start with a and so on... so a, b, c...
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32         |   a reference
// &'a i32      |   a reference with an explicit lifetime
// &'a mut i32  |   a mutable reference with an explicit lifetime
