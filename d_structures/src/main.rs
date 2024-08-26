/*
- Struct = structures
- Data type that contains multiple pieces of data.
- Each piece of data, called a field, must be populated.
- Makes working with data easier as the same ones can be grouped together.
*/

/*
use crate::Flavors::Orange;

#[derive(Debug)]
enum Flavors {
    Blueberry,
    Orange,
    Mango,
}

struct DrinkInformation {
    flavor: Flavors,
    fluid_oz: f64,
}

fn display_information(drink_information: DrinkInformation) {
    match drink_information.flavor {
        Flavors::Blueberry => println!("Flavor: {:?}", drink_information.flavor),
        Flavors::Orange => println!("Flavor: {:?}", drink_information.flavor),
        Flavors::Mango => println!("Flavor: {:?}", drink_information.flavor),
    }
    println!("Ounces: {:?}", drink_information.fluid_oz);
}

fn main() {
    let a = DrinkInformation {
        flavor: Flavors::Blueberry,
        fluid_oz: 1.35,
    };
    let b = DrinkInformation {
        flavor: Flavors::Orange,
        fluid_oz: 1.3,
    };
    let c = DrinkInformation {
        flavor: Flavors::Mango,
        fluid_oz: 1.357,
    };
    display_information(c);
}
*/

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("HamzaAnwar"),
        email: String::from("hamzaanwar93@outlook.com"),
        is_active: true,
        sign_in_count: 5,
    };
    let name = user1.username;
    println!("{name}");

    // We can update fields of the struct but for that, make the struct mutable.
    // Remember, we cannot make only a single field of the struct mutable. We need to make entire struct mutable.
    let mut user2 = User {
        username: String::from("Hamza Anwar"),
        email: String::from("hamxaanwar@live.com"),
        is_active: false,
        sign_in_count: 3,
    };
    println!("User2 count is {}", user2.sign_in_count);
    user2.sign_in_count = 10;
    println!("User2 count after update is {}", user2.sign_in_count);

    let user3 = build_user(String::from("ali_unknown@xyz.com"), String::from("Ali"));
    println!("{}", user3.username);

    // Struct can also contain fields from other structs such as:
    let user4 = User {
        email: String::from("abc@xyz.com"),
        username: String::from("ShanAnwar"),
        ..user1
    };

    // We also have tuple structs:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

// lets make another function to update and build our struct "User".
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_active: true,
        sign_in_count: 1,
    }
}
