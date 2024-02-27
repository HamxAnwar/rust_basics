/*

Could be one of the two things:
    - Some data of specified type.
    - Nothing.
It could be used in scenarios where the data is not required entirely or is unavailable at the time.
Some use cases for option type:
    - Unable to find the data.
    - Ran out of items in a list.
    - Form field not filled out.
An option type is part of Rust and is defined as an enum type with two variants:

enum Option<T> {
    Some(T),
    None,
}

Normally we need to access enums using Options::Variant.
In Option type, we don't need to specify Option:: each time and can directly use Some(value) or None without specifying the Option::

*/

struct Student {
    name: String,
    locker_assignment: Option<i32>,
}

fn main() {
    let student_1 = Student {
            name: "Hamza".to_owned(),
            locker_assignment: Some(60),
    };

    let student_2 = Student {
        name: "Ali".to_owned(),
        locker_assignment: None,
    };

    println!("Name = {:?}", &student_1.name);
    match student_1.locker_assignment {
        Some(num) => println!("Locker assignment: {:?}", num),
        None => println!("No locker assigned"),
    }

    println!("Name = {:?}", &student_2.name);
    match student_2.locker_assignment {
        Some(num) => println!("Locker assignment: {:?}", num),
        None => println!("No locker assigned"),
    }
}