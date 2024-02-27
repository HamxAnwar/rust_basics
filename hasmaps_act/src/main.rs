use std::collections::HashMap;

fn main() {
    let mut stocks = HashMap::new();
    stocks.insert("Chairs", 5);
    stocks.insert("Beds", 3);
    stocks.insert("Tables", 2);
    stocks.insert("Couches", 0);

    let mut total_stock = 0;

    for (name, quantity) in stocks.iter() {
        let qty = if quantity == &0 {
            "Out of stock".to_owned()
        } else {
            format!("{:?}", quantity)
        };
        total_stock = total_stock + quantity;
        println!("Item: {:?}, Quantity: {:?}", name, qty);
    }

    println!("Total number of items: {:?}", total_stock);
}