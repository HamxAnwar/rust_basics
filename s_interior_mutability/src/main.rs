// Interior mutability is a design pattern in rust that allows mutability even when there are immutable references to that data.
// It uses unsafe code.
// Unsafe code is not checked at compile time for memory safety.
// So instead of compile time, we can enforce the memory safety rules on the run time.

// --- RefCell smart pointer ---
// Represents single ownership over data just like box pointer.
// Difference:
// - Box pointer enforces borrowing rules at compile time.
// - RefCell pointer enforces borrowing rules at runtime. (Breaking these rules at runtime will make the program panic and exit.)
// The drawback is, errors are caught at the compile time easily and there is no runtime cost.
// Also for single threaded programs.

// Box and RefCell pointers enables single owners of the data while the Rc pointer enables mutiple owners of the data.
// Box pointer allows immutable and mutable borrows checked at compile time while Rc allows only immutable borrows to be checked at compile time. RefCell pointer allows both mutable and immutable borrows checked at runtime.
// Since RefCell pointer allows mutable borrows checked at runtime, we can mutate the value inside the RefCell even when RefCell is immutable.
// For example in the code below:
// - We can not mutable borrow a to b as a is immutable.
// - Also we cannot change c through d since d is an immutable reference though c is mutable itself.

/*
fn main() {
    let a = 4;
    let b = &mut a;

    let mut c = 19;
    let d = &c;
    *d = 20;
}
*/

// To to lib.rs for further information.

// -------------------------------------------------------------//

// Next we can also combine Rc and RefCell to get mutable borrows of more than one owned references.
// For example the code below:

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
