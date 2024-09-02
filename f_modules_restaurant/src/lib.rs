// This is a library crate which manages a restaurant.
// Two parts of the restaurant:
//  - front of the house
//  - back of the house
//  - So we have two modules.
//  - Modules can contain other modules.
//      - The module front_of_house contains hosting and serving module.
//      - We have functions inside the module.
//  - Modules can contain in themselves:
//      - modules
//      - structs
//      - enums
//      - contants
//      - traits
//      - and so on...
//  - helps keep code organized.



mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

// Note:
//  - If we have a module that is private, to make it accessible to external functions such as below, we need to make it public using the "pub" keyword.
//  - Making modules public doesn't make the functions inside it public, we have to explicitly make them public.

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}


//----------------------------------------------------------------------------

fn serve_order(){}

mod back_of_house {
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();       // Super keyword allows us to reference the parent module since the serve keyword is outside the back_of_house module.
    }

    fn cook_order(){}
}

