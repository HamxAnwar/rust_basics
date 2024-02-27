#[derive(Debug)]
enum Color {
    Black,
    Red,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Black => println!("Black"),
        }
    }
}

#[derive(Debug)]
struct Dimensions {
    width: f64,
    height: f64,
    length: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width = {:?}", self.width);
        println!("height = {:?}", self.height);
        println!("length = {:?}", self.length);
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}
impl ShippingBox {
    fn print_box(&self)  {
        self.dimensions.print();
        self.color.print();
        println!("Weight: {:?}", self.weight)
    }
}

fn main() {
    let box_characters =  ShippingBox { dimensions: Dimensions {
        width: 22.4,
        height: 81.4,
        length: 87.5,
    },
    weight: 22.7,
    color: Color::Red,
    };
    box_characters.print_box();
}