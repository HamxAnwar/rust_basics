/*
Tuple:
- A type of record.
- Useful to returns pairs of data from functions.
- Can be destructured easily into variables.
*/

#[derive(Debug)]
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

fn main() {
    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("{:?}, {:?}", numbers.0, x);
    println!("{:?}, {:?}", numbers.1, y);
    println!("{:?}, {:?}", numbers.2, z);

    // We can also mix multiple data types.

    let (employees, access) = ("Hamza", Access::Full);
    println!("{:?} {:?}", employees, access)
}