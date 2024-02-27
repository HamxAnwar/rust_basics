fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn display(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = add(12,12);
    display(result);
}