// Closures are like functions but without names.
// They can be stored in variables, passed around and passed in as input parameters to a function and they capture the variables inside the scope in which they are defined.

// Imagine we are building a code for a fitness app and the backend is made in rust. We have a function which is very expensive to run, called simulated_expensive_calc.
// To understand closures, lets go through the following code example:

use std::thread;
use std::time::Duration;

/*
fn simulated_expensive_calc(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/

/*
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Today, do {} pushups!", simulated_expensive_calc(intensity));
        println!("Next, do {} situps!", simulated_expensive_calc(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to sleep!");
        } else {
            println!(
                "Today, run for {} minutes",
                simulated_expensive_calc(intensity)
            );
        }
    }
}
*/

// We can see that in the above commented generate_workout function, simulated_expensive_calc(intensity) is called multiple times and since it is an expensive functions and runs everytime, it is a bad idea. We need to limit the use of this function. We can do this using a variable to store the result of the function and pass that result everytime as below.

/*
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calc(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to sleep!");
        } else {
            println!("Today, run for {} minutes", expensive_result);
        }
    }
}
*/

// The above code solves the problem of expensive function's repeated execution but we have another problem. Our expensive function runs in every fn call irrespective of if the expensive function is required or not... Such as in the else-if statement.

// What we can do is want to define our code in a single place and use only when required. We will do this using closures.
// In the below code, we define the variable expensive closure and set it equal to a closure. A closures are anonymous functions and one of the difference between them and normal functions are parameters are not defined in parenthesis but inside  " = |parameters|".
// lets call this closure inside our function.

/*
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }; // Remember that the expensive closure variable is not storing the return value but the closure itself.
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to sleep!");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity));
        }
    }
}
*/

// Notice that we do not have to annotate the type of input and output parameters in our closure unlike normal functions.
// The compiler do it but if we want, we could specify the types explicitly.

// In the above code, we can see that in the first if block, we are calling the closures twice.
// We can solve this problem by saving the closure output into a variable at the start of the if block. Another way to do it more effectively is through the memorization pattern using structs.
// While using structs, enums or even function parameters etc for closures, we need to use generics and trait bounds.

/*
struct Cacher<T>
where
    T: Fn(u32) -> u32,
    // Remember that all closures and functions implements one of the three fn traits: Fn, FnMut or FnOnce.
{
    calculation: T,     // It is a closure
    value: Option<u32>, // Option type because during initialization, it will have a value of None and then we pass a u32 value.
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // constructor function
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // this is where caching happens
                v
            }
        }
    }
}

// Now lets put our struct-closure definition inside our generate_workout function below:

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }); // Remember that the expensive closure variable is not storing the return value but the closure itself.

    if intensity < 25 {
        println!("Today, do {} pushups!", cache_result.value(intensity));
        println!("Next, do {} situps!", cache_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to sleep!");
        } else {
            println!("Today, run for {} minutes", cache_result.value(intensity));
        }
    }
}

// Now if a user needs to know about a different workout routine, he will call another function named generate_workout as below:

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7; // Not in the scope of this project so we neither generate a random number nor we simulate any intensity but just give hardcoded values.

    generate_workout(simulated_intensity, simulated_random_number);
}
*/

// Caching values is generally useful.
// In the above code, there are two problems:
//  - No matter what the arg input parameters is, the cache will return only single value which was first assigned to the value in the struct.
//    We can try to use a hashmap instead of single value to solve this problem.
//    The keys of the hashmap will be the arg passed into value and the values will be the result of the "let v = (self.calculation)(arg)".
//    Then we need to look inside the hashmap for the passed value, if it exist, return that. Otherwise, run the above statement and return the value of v.
//  - We are using hard coded types.
//    Use generics instead of hard coded values.

// Lastly capturing environment using closures.
// Unlike functions, closures have access to the variables that are defined inside the scope in which closures are defined.
// Such as in the below code, the closure has access to x.
// If we had use a function instead of a closure, while calling the function, there would be error since the function can't capture a dynamic environment.
// Due to above, closure are more memory hungry that functions.

fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;

    /*
    fn equal_to_x(x: i32) -> bool {
        z == x
    }
    */

    let y = 4;
    assert!(equal_to_x(y)); // This will panic if x and y are not equal. If equal, it will pass.
}

// Closures capture values from environment in three ways:
//      These are same as functions capture input parameters.
//          - By taking ownership       -   FnOnce
//          - By borrowing mutably      -   FnMut
//          - By borrowing immutably    -   Fn
// The rust compiler decides which if the above to use by looking at the program and the usage of the values inside the closure's environment.
// Thus, it should be noted that the closure doesn't take the ownership of the variable x.
// However, we can force the closure to take the ownership of the values it uses inside the environment by using the "move" keyword as below:

/*
fn main() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    println!("Can't use x here: {:#?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
*/