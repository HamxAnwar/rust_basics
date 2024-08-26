// Make a code using structs to calculate the area of a triangle.

struct Rectangle {
    height: u32,
    width: u32,
}

/*
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
*/

// Instead of using a separate function for calculating area, we can see that the function for area is closely related to our struct so we can define an implementation.
impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// We can also define associated functions and not methods/implementations.
// We can define this function in the above impl block but just for an example that structs allow multiple impl blocks, we define other.
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

/*
fn main() {
    let rectangle = Rectangle {
        height: 10,
        width: 12,
    };
    let area = calculate_area(&rectangle);
    println!(
        "Area of given triangle = {} x {} = {}",
        rectangle.height, rectangle.width, area
    );
}
*/

fn main() {
    let rectangle_1 = Rectangle {
        height: 10,
        width: 12,
    };
    let rectangle_2 = Rectangle {
        height: 3,
        width: 9,
    };
    let area = rectangle_1.calculate_area();
    println!(
        "Area of given triangle = {} x {} = {}",
        rectangle_1.height, rectangle_1.width, area
    );

    println!(
        "Rectangle 1 can hold rectangle 2: {}",
        rectangle_1.can_hold(&rectangle_2)
    );
    println!(
        "Rectangle 2 can hold rectangle 1: {}",
        rectangle_2.can_hold(&rectangle_1)
    );

    let our_square = Rectangle::square(3);
    println!("The area of our square: {}", our_square.calculate_area());
}
