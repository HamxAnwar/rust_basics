// Object Oriented Programing in RUST
// Rust has many features that could be said to be OOP oriented but RUST have also features that are from functional languages.
// OOP in RUST can be discussed with main three features:
//      - Objects
//      - Encapsulation
//      - Inheritance
// Objects are basically composed of the data and methods that operate on that data.
// Structs and enums hold data and we can use impl blocks for methods on them. They are not objects but do provide the same functionality.
// Encapsulation is that implementation details of an object is hidden from the code, using that object.
// Similar can be done by using the pub keyword to make our types, struct, enums and implementations public.
// Inheritance is the ability of one object to inherit from another object's definition, gaining the data and behavior of the other object without defining that data and behavior itself.
// RUST doesn't have this ability.
// We can't define a struct that have the data and ability of another struct.
// However, rust has some other tools that we can use due to various reasons, as below:
//      - Code sharing => use default trait implementations.
//          - Traits can, for now, only define methods and not fields.
//      - For polymorphism - It allows us to substitute various objects for each other at runtime if they share same characteristics, such as from the parent class.
//          - Suppose, a base class called vehicle and subclasses that inherit from that such as trucks, car, etc.
// Rust takes a different approach to the above.

fn main() {
    println!("Hello, world!");
}
