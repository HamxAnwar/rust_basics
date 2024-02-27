fn coord() -> (i32, i32) {
    (5, 5)
}

fn main() {
    let (x, y) = coord();
    if y > 5 {
        println!("{:?} is greater than 5", y);
    }
    else if y < 5 {
        println!("{:?} is less than 5", y);
    }
    else {
        println!("{:?} is equal to 5", y);
    }
}
