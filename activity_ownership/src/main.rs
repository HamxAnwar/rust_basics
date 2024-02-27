use std::process::id;

struct GroceryItems {
    quantity: i32,
    id_number: i32,
}

fn display_quantity(x: &GroceryItems){
    println!("Quantity = {:?}", x.quantity);
}

fn display_id(x: &GroceryItems){
    println!("ID = {:?}", x.id_number);
}

fn main() {
    let grocery_item = GroceryItems {
        quantity: 50,
        id_number: 3,
    };
    display_id(&grocery_item);
    display_quantity(&grocery_item)
}
