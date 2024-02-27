#[derive(Debug)]
struct Backstage {
    name: String,
    price: i32,
}

#[derive(Debug)]
struct Vip {
    name: String,
    price: i32,
}

#[derive(Debug)]
enum Tickets {
    Backstage(Backstage),
    Vip(Vip),
    Standard(i32),
}

fn main() {
    let tickets = [
        Tickets::Backstage(Backstage {
            name: "Hamza".to_owned(),
            price: 50,
        }),
        Tickets::Vip(Vip{
            name: "Jim".to_owned(),
            price: 42,
        }),
        Tickets::Standard(100),
    ];

    for ticket in tickets{
        match ticket {
            Tickets::Backstage(Backstage{name,price}) => println!("For Backstage - Name: {:?}, Price: {:?}", name, price),
            Tickets::Vip(Vip{name,price}) => println!("For VIP - Name: {:?}, Price: {:?}", name, price),
            Tickets::Standard(price) => println!("For Standard - Price: {:?}", price),
        }
    }

}
