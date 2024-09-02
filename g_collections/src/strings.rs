/*
    - In rust, strings are stored as UTF-8 encoded bytes.
    - American Standard Code for Information Interchange (ASCII) is used to encode bytes to strings or strings to bytes.
    - It stores characters in only a byte and 7 bites are used represent a charater.
    - Only english characters, some symbols and few commands can be represented by ASCII.
    - Since, ASCII only represents the english characters, other countries created their own standards.
    - This is problametic... How does a program know while parsing a collection of byte that which encoding to use?
    - Thus, solving this problem, Unicode was created.
    - Unicode is a universal character set. It stores characters and symbols from various languages as well as emojis.
    - Backward compatible with ASCII - First 128 characters of Unicode are ASCII characters.
    - UTF-8 is a variable width character encoding for unicode.
    - It is variable width because in UTF-8, each character can be represented by 1 byte, 2 bytes, 3 bytes or 4 bytes.
    - ASCII represents in only 1 byte.
    - Since strings are UTF-8 encoded, we can write its content as english or any other language, symbols and emojis.
*/
use unicode_segmentation::UnicodeSegmentation;

fn main(){
    let s1 = String::new();                 // Like vectors, initialized a new string with out specifying its contents.
    let s2 = "A string slice";              // A string slice.
    let s3 = s2.to_string();                // Converting a string slice to an owned string.
    let s4 = String::from("Owned string");  // Initializing an owned string with its content.

    // Appending to a string.

    let mut s5 = String::from("foo");
    s5.push_str("bar");         // Appending a text or character, we use string slices because we do not want to take ownership of that string.
    s5.push_str("!");
    println!("The content of s5 is {:#?}", s5);

    // We can also achieve the same by using concatenation or the format macro..

    let s6 = String::from("Hello ");
    let s7 = String::from("World!");
    let s8 = s6 + &s7;
    let s9 = format!("{}{}", s6, s7);
    println!("The new string in s8 is {:#?}", s8);
    println!("The new string in s8 is {:#?}", s9);

    // In the format macro, the ownership of both variables s6 and s7 is not transferred.

    // Moreover, when we want to iterate over a string or get a character of a string, we cannot specify a simple index.
    // Since unicode characters can be 1-4 bytes and a word in various languages will be having different sizes.
    // A word in unicode can be represented in 3 different ways.
    //  - Bytes: A collection of bytes.
    //  - Scalar values: These are the building blocks of a word. So they can be a character or parts of a character. This is the char type in rust.
    //  - Graphemes cluster: This is what we consider a character.

    let hello = String::from("Hello");
    for b in hello.bytes() {
        println!("{}", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    // For Grapheme clusters, rust doesn't include them by default and we need to use external crate.

    for g in hello.graphemes(true) {
        println!("{}", g);
    }
}
