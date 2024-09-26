// In majority of cases, ownership is clear.
// But there are cases where we might have multiple owners.
// For example, we have a graph with multiple edges, pointing to the same node.
// Conceptually that node is owned by all the edges.
// To enable this multiple ownerships, we can use a reference counting (RC) smart pointer which will help track the number of references to a value.
// When there exist no more references, the value gets cleaned up.
// RCSP are used when we have to allocate a value on the heap and multiple parts of our program needs to access that value and we donot know, which part will use it last at compile time.
// In case we know which part uses it lastly, we can make this last part owner of the value and ownership rules will be applied.
// This RCSP in this tutorial is only useful for single threaded programs.
// Exampling for RCSP, we will take cons-list for example.
// - Take two cons-lists
// - The two lists should point to a single third list.
// - Thus the two lists share ownership of the third list.
// - b and c points to a.

/*
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(3, Box::new(Cons(5, Box::new(Nil))));
    let b = Cons(5, Box::new(a));
    let c = Cons(4, Box::new(a));
}
*/

// In the above code, a is a box list.
// b and c points to a.
// but it states an error in c where "use of moved value: `a` value used here after move".
// To share the ownership, we need to use the RCSP.
// So what we can do it intead of a Box pointer, we can use a RCSP.

/*
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(5, Rc::new(Nil)))));
    let b = Cons(5, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
*/

// To achieve the sharing list, we used Rc from std::rc::Rc instead of Box.
// Then b and c will have Rc::clone and a reference to a.
// Rc::clone doesn't make deep copies of the data but only increments the reference count.
// To analyze the effects of creating Rc, run the code below.

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(3, Rc::new(Cons(5, Rc::new(Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(5, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
}
