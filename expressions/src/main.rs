// Rust is an expression based language: Most things are evaluated and return some value.
// Expression values can be used for nesting logic.

// For example

/*

let my_num = 3;
let is_lt_5 = if my_num < 5 {
    true
} else {
    false
};
let is_lt_5 = my_num < 5;

*/

fn message(boolvar:bool) {
    match boolvar {
        true => println!("Its big!"),
        false => println!("Its small!"),
    }
}

fn main() {
    let x = 100;
    let boolvar = x >=100;
    message(boolvar);
}
