// The below program gets a vector variable with numbers.
// We want to check is the vector is empty or if there are numbers in it.
// For it and other purposes, std library has many built-in functions.
// To access its documentation, we can write in terminal:
//      - rustup doc
// In the docs, we can see the is_empty method to achieve what we wanted.
// I have modified the code with loop for practice purposes.

fn main() {
    let mut numbers = vec![1, 2, 3];
    let mut x=3;
    while x > -1 {
        match numbers.is_empty() {
            true => println!("Dont have entities"),
            false => println!("Has numbers"),
        }
        numbers.pop();
        x -= 1
    }
}
