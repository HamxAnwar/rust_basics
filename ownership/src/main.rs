/*
Every program must track memory.
If they fail to do so, a leak will occur.
Every language has its own method of doing it. Rus uses ownership.
The owner is simply a function in rust.
The memory can be either moved or borrowed from the owner.
*/

// --------------------------------

// #[derive(Clone, Copy)]
/*enum Light {
    Bright,
    Dull,
}

fn display_light(light: Light) {
    match light {
        Light::Bright => println!("Bright!"),
        Light::Dull => println!("Dull!")
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(dull);
    display_light(dull);
}*/

// The above code gives error as the use of display_light function two times throws an ownership error.
// The dull variable ownership moves from main to the display_light function.
// Upon successful execution, the display_light function deletes the dull variable.
// Calling it a second time through error.

// ------------------------------------------------------------------

// We can tackle the above by borrowing as below:

enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright!"),
        Light::Dull => println!("Dull!")
    }
}

fn main() {
    let dull = Light::Dull;
    display_light(&dull);
    display_light(&dull);
}