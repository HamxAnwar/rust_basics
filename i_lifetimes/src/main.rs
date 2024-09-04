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

/*
fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    eprintln!("result = {:#?}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // We can name a lifetime anything we want but the convention is to start it with ' and start with a and so on... so a, b, c...
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// &i32         |   a reference
// &'a i32      |   a reference with an explicit lifetime
// &'a mut i32  |   a mutable reference with an explicit lifetime

// What the lifetimes reflect is what is the lifetime of our returned value in relation to the lifetimes of the variables/parameters.
// Basically the lifetime of the return will be same as the smallest lifetime of the parameters/arguments.
// In this case, the lifetime or both arguments i.e. string1 and string2 are equal, the return's lifetime will be equal to both.
// Lets consider another example below with different lifetimes:

/*
fn main() {
    let string1 = String::from("abcd"); // Lifetime equal to the complete main function

    {
        let string2 = String::from("xyz"); // Lifetime equal to the inner scope
        let result = longest(string1.as_str(), string2.as_str()); // Result will have a lifetime equal to that of string2 as it has the smallest lifetime compared to string1.
        eprintln!("result = {:#?}", result);
    }
    // eprintln!("result = {:#?}", result);         // uncommenting this throws error...
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // We can name a lifetime anything we want but the convention is to start it with ' and start with a and so on... so a, b, c...
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// The lifetime of return value always have to be tied to one of the variables.
// That is because if we have to return the reference of a value, it should be what has been passed in.
// We cannot return reference of any value that had been defined or initialized inside the function because the scope of that variable will be the function and will be dropped as soon as the function completes execution. For example:

/*
fn longest() -> &str {
    let result = String::from("This function will throw an error as the scope of the string ends with this function.")
    result.as_str()             // Error!!!
}
*/

// Other option that we have is to return an owned type instead of passing a reference to a type.
// Next we would like to discuss structs with lifetime annotations.
// This is important if a struct is given a reference. The below example shows that the lifetime of our struct is the same as our first_sentence variable.

/*
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Hamza. I am one of the legions!");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find the message...");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
*/

// Also remember if there is a single variable being passed, the compiler already determines the lifetime and we might not have to specify the lifetime explicitly.

/*
fn main() {}
fn first_word<'a>(s: &'a str) -> &'a str {
    // This can also be written as below without any error:

    // fn first_word(s: &str) -> &str

    // In the above commented out statement, the compiler automatically estimates the lifetime as it considers three rules below.
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
*/

// Rules:
//  - Each parameter that is a reference gets its own lifetime parameter.
//  - If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
//  - If there are multiple input lifetime parameters, but one of the is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

// So going back to the ImportantExcerpt example, we discuss below:

/*
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Hamza. I am one of the legions!");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find the message...");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
*/

// Remember that LTA are a form of generic, so their declaration are same as generics.
// In the above code, we specified the lifetime annotations in the impl but not the functions inside it.
// The reason for that is, if we go through the rules for lifetime:
//  - &self and announcements, both will get a lifetime.
//  - According to rule two, this case have two impl lifetimes variables instead of one.
//  - The function contains a &self, so the lifetime for the return will be equal to that of &self.
// The function can also be written as:

/*
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {       // we donot specify <'a> with function as we already have it above with impl.

        println!("Attention please: {}", announcement);
        self.part
    }
}
*/

// Lastly we have a special lifetime called the static lifetime.

fn main() {
    let s: &'static str = "This has a static lifetime.";
}

// A static lifetime variable can live for the duration of the entire program.
