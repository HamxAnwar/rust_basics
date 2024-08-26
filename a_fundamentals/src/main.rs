use std::iter::Sum;

// These are pure basics of Rust-Lang
// Variables
// Variables are immutable by default.
fn main() {
    println!("Hello, world!");
    let _x = 5;
    // We cannot update the value of x below this since it is immutable. To make it modifiable, we can use the keyword mut as below:
    let mut _x = 29;
    _x = 1;
    println!("The value of x is {}", _x);

    // Constants
    // Written in capitalized letters.
    // consts cannot be mutable.
    // consts must type annotated.
    // consts cannot be put as a return value to any function.
    const _A: u8 = 2; // This cannot be written as const mut _A: u8 = 2;

    // Shadowing
    // Instead of making variables mut, we can use shadowing to reassign values to variables.
    // 1. During both reassignments, the variables are immutable.
    // 2. We could change data types of the variables while shadowing.
    // Example: line 8.

    // Scalar datatypes
    // 1. Integers
    //      u8, u16, u32, u64, u128, arch = usize
    //      i8, i16, i32, i64, i128, arch = isize
    //      While working with integers, the max of the datatype would wrap around.
    let a = 97_222; //Decimal
    let a = 0xff; //Hex
    let a = 0b1111_0000; //Binary
    let a = b'A'; //Byte (u8 only)

    // 2. Floats
    let x = 3.45;

    // 3. Booleans
    //      true or false

    // 4. Characters
    //      Represents a unicode character
    //      Written in single quotes.

    // Compound datatypes
    // 1. Tuples
    //      Declared in parethesis
    //      Can have multiple data types.
    let tup = ("I am game!", 100_000);

    //      Can be assigned as follows:
    let (x, y) = tup;

    //      We can also assign single values:
    let x = tup.0;
    let y = tup.1;

    // Arrays
    //      Arrays are single datatype with fixed length in square brackets.
    //      Use a[1] to access elements of array a.
    let an_array = [1, 2, 3, 4];
    let also_an_array = [0; 8]; // This means create an array of zeroes with 8 elements.

    // Statements vs Expressions
    //      A statement doesn't return anything while an expression returns some value.
}

fn function_example() {
    let x = 12;
    let y = 'a';

    // we can return the sum of x and y in the following ways:
    // 1.
    let sum = x + y;
    return sum;

    // 2.
    let sum = x + y;
    sum

    // 3.
    x + y
}
