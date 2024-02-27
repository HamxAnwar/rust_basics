// impl is the implementation block.
// If we have a struct and a function that processes that struct, we dont need to implement them seperately.
// We can use impl block for that as below..

// ----------------------------------------------------------------------------- //

/*struct Temperature {
    degrees_f: f64,
}

fn print_temp(temp: Temperature) {
    println!("Temperature = {:?}", temp.degrees_f)
}

fn main() {
    let temp = Temperature { degrees_f: 100.0 };
    print_temp(temp);
}
*/
// ----------------------------------------------------------------------------- //
// instead we can:

// ----------------------------------------------------------------------------- //

/*struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn print_temp(temp: Temperature) {
        println!("Temperature = {:?}", temp.degrees_f)
    }
}

fn main() {
    let temp = Temperature { degrees_f: 100.0 };
    Temperature::print_temp(temp);
}*/

// ----------------------------------------------------------------------------- //
// A better way to do the above is:

/*struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn print_temp(&self) {
        println!("Temperature = {:?}", self.degrees_f)
    }
}

fn main() {
    let temp = Temperature { degrees_f: 100.0 };
    temp.print_temp();
}*/

// ----------------------------------------------------------------------------- //
// We can add another function in the implementation block such as one for freezing temps.

struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    fn freezing() -> Self {             // Self with capital 'S' means that a new value of temperature is being created.
        Self { degrees_f: 0.0 }
    }
    fn print_temp(&self) {              // self with small 's' means that the value of temperature is already defined somewhere.
        println!("Temperature = {:?}", self.degrees_f)
    }
}

fn main() {
    let temp = Temperature { degrees_f: 100.0 };
    temp.print_temp();
    let cold = Temperature::freezing();
    cold.print_temp()
}