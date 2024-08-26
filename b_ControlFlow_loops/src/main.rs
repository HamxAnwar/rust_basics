// Control flow statements in RUSTLang.

fn main() {
    // Loop 1:
    //      Continuous looping.
    //      Needs a break statement.
    let mut counter = 0;
    let c = loop {
        println!("{counter}");
        counter += 1;

        if counter == 10 {
            // We can return values from loops as follows:
            break counter;
        }
    };
    println!("The last value of counter is: {c}");

    // While loop:
    let mut condition = 3;
    while condition > 0 {
        condition -= 1;
    }
    println!("The value of condition is {condition} now.");

    // For loop:
    //      To loop through an array, a range, etc.
    let a = [1, 2, 3, 4, 5];
    for elements in a.iter() {
        println!("The elements of array is {elements}");
    }

    // For a loop over range, the range excludes the max number.
    for elements in (1..10) {
        println!("The elements in range is {elements}");
    }
}
