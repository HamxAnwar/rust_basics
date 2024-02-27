// Hashmaps are data structure.
// Useful when we exactly know what we are looking for.
// Key:Value pairs
// Similar to dictionaries.
// To define a new hashmap, we can use the new function in hashmap.
// Remember the data in a hasmap is stored randomly. So we do not get the data in the same order we save it.

use std::collections::HashMap;

/*fn main() {
    let mut people =  HashMap::new();
    people.insert("Hamza", 29);
    people.insert("Ehtisham", 27);
    people.insert("Ahmed", 37);
    people.insert("Ali", 29);
    people.insert("Javad", 50);
    people.remove("Javad");

    match people.get("Ahmed") {
        Some(age) => println!("age: {:?}", age),
        None => println!("No result found"),
    }

    for (person, age) in people.iter() {
        println!("Person: {:?}, Age: {:?}", person, age);
    }
    for person in people.keys() {
        println!("Person: {:?}", person);
    }

    for age in people.values() {
        println!("Age: {:?}", age);
    }
}
*/

// Demo:

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents {content: "Stuff".to_owned()});
    lockers.insert(2, Contents {content: "Makeup".to_owned()});
    lockers.insert(3, Contents {content: "Books".to_owned()});

    for (locker_number, content) in lockers.iter() {
        println!("Locker number: {:?}, Content: {:?}", locker_number, content);
    }
}