// Hashmaps are used to store key:value pairs.
// These key and values can be of any type.
// It uses a hashing function to determine how to keep those keys and values in the memory.

use std::collections::HashMap;

fn main() {
    let blue = String::from("Blue");
    let red = String::from("Red");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(red, 100);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:#?}", score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // If we want to insert a new value, remember:

    let new_scores = HashMap::new();
    new_scores.insert(String::from("Blue"), 10);
    new_scores.insert(String::from("Blue"), 100);

    // The above will replace the value of blue from 10 to 100.
    // Instead, if we dont want to replace the value, we can do the following:

    new_scores.entry(String::from("Yellow")).or_insert(20);     // This will check for the Yellow key, if it doesn't exist, will make an entry for the key-value pair. Thus, makes an entry for the key Yellow wince it doesn't exist.
    new_scores.entry(String::from("Yellow")).or_insert(30);     // This will do nothing as Yellow key already exists.
    


    let text = "name of my name is Hamza";
    // I want to populate the hashmap using the words in text above and how many times, the word repeats as its value.

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);           // Returns an option with either a Some(value) or None.
        *count += 1;        // The above also returns a mutable reference to our value. So we de-referenced it and added 1 to it.
    }

    println!("{:#?}", map);
}
