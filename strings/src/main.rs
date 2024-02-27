// There are two types of strings:
// Strings  - owned
// &str     - borrowed string slice
// We must use an String to store in structs.
// We use &str when passing to a function.
// You cannot store a string slice in an struct.
// if we use variable = "Some string";   it always make a string slice.

/*

fn print_it(data: &str) {
    println!("{:?}", data);
}

fn main() {
    print_it("a string slice");
    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");
    print_it(&owned_string);
    print_it(&another_owned);
}

*/

struct LineItem {
    name: String,
    count: i32,
}

fn print_item_name(name: &str){
    println!("Name: {:?}", name);
}

fn main() {
    let items = vec![
        LineItem {
            name: "Cereal".to_owned(),
            count: 5,
        },
        LineItem {
            name: String::from("Juice"),
            count: 9,
        },
    ];

    for item in items {
        print_item_name(&item.name);
        println!("Count: {:?}", item.count);
    }
}
