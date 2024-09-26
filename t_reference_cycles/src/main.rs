// Rustlang makes it difficult to create memory leaks but not impossible.
// We can use Rc or RefCell to make in a program, items that refer to each other in a cycle, hence introducing memory leaks.

fn main() {
    println!("Hello, world!");
}
