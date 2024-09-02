mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("Mangoes"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Imagine we want to change the toast now from rye to wheat.
    // Inside a public struct, the fields of the struct are not public and are private unless explicitly specified.

    meal.toast = String::from("wheat");
}

pub fn eat_at_restaurant_enum(){
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // We have to make the enum public but not its variants. Variants are what the enum is. Enums wouldn't make much sense if their variants are private, thus they are public for public enums by default.

}
