/*
- Struct = structures
- Data type that contains multiple pieces of data.
- Each piece of data, called a field, must be populated.
- Makes working with data easier as the same ones can be grouped together.
*/

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
