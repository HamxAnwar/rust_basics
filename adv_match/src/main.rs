enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other), // We have specified other instead of _ because in rust, _ generally means ignore the value. So we specify something more ongoing as we are not ignoring the value but printing it.
    }

    let flat = Discount::Flat(2);
    match flat {
        /* We can implement through various ways.
        Discount::Flat(2),
        Discount::Flat(_),
        Discount::Flat(variable_name),
        */
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount),
        _ => (), // Empty parenthesis means we return nothing.
    }

    // We can also match on a struct.

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 32,
    };
    
    match concert {
        // The Ticket {} means we are matching a struct.
        // The .. means ignore everything else since we are not concerned about what the price name is.
        // We can also set the value as:
        Ticket { price: 32, event} => println!("event @ 32 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}