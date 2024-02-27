fn main() {
    let age = 20;
    if age >= 18 {
        println!("Purchase eligible")
    }
    else {
        println!("Purchase denied. {:?} is less than 18.", age)
    }
}
