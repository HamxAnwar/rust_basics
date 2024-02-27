/*
- Match expressions are similar to if..else statement but that the match expressions should
    be exhaustive, i.e. all options must be accounted for.
- Match statements are checked by the compiler while if...else are not.
- Prefer match if using a single variable.
*/

fn main() {
    let my_name = "Hamza";
    match my_name {
        "Hamza" => println!("This is my name"),
        "Jimmy" => println!("Not my name"),
        _ => println!("Let's see name"),
    }

    let boolean = false;
    match boolean {
        true => println!("it's true"),
        false => println!("it's false"),
    }

    let integer_number = 1;
    match integer_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
