/*
- Enumeration: Data that can be one of various different possibilities.
- Each possibility is called a variant.
- More robust program when combined with match.
- Can only be one variant at a time.
- enum is declared outside of our main function.
*/

use crate::Colors::White;

enum Direction{
    Left,
    Right,
    Forward
}

#[derive(Debug)]
enum Colors{
    White,
    Black,
    Red,
    Pink,
    Blue,
}

fn print_color(x: Colors) {
    match x {
        Colors::White => println!("The color is {:?}", x),
        Colors::Black => println!("The color is {:?}", x),
        Colors::Red => println!("The color is {:?}", x),
        Colors::Pink => println!("The color is {:?}", x),
        Colors::Blue => println!("The color is {:?}", x),
    }
}

fn main() {
    // Demo
    let go = Direction::Right;
    match go {
        Direction:: Left => println!("Go left"),
        Direction:: Right => println!("Go right"),
        Direction:: Forward => println!("Go forward"),
    }

    // Activity
    let color = Colors::White;
    print_color(color)
}
