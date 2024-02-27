#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new_adult(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self { name: name.to_string(), age })
        }
        else { Err("The user is less than 21 years old.") }
    }
}

fn main() {
    let adult = Adult::new_adult("Hamza", 30);
    let child = Adult::new_adult("Ahmed", 20);
    match adult {
        Ok(adult) => println!("{:?} is {:?} years old.", adult.name, adult.age),
        Err(e) => println!("{:?}", e),
    }
    match child {
        Ok(child) => println!("{:?} is {:?} years old.", child.name, child.age),
        Err(e) => println!("{:?}", e),
    }
}
