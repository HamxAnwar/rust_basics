struct Person {
    name: String,
    age: i32,
    fav_color: String,
}

fn print_nc(name: &str, color: &str) {
    println!("Name: {:?}", name);
    println!("Favourite color: {:?}", color);
}

fn main() {
    let people_list = vec![
        Person {
            name: "Hamza".to_string(),
            age: 7,
            fav_color: "Ember".to_string(),
        },
        Person {
            name: String::from("Ali"),
            age: 92,
            fav_color: "Indigo".to_string(),
        },
        Person {
            name: "Ehtisham".to_string(),
            age: 5,
            fav_color: "White".to_string(),
        },
        Person {
            name: String::from("Ahmed"),
            age: 25,
            fav_color: "Indigo".to_string(),
        },
        Person {
            name: String::from("Jameel"),
            age: 10,
            fav_color: "Indigo".to_string(),
        },
    ];
    for person in people_list {
        if person.age <= 10 {
            print_nc(&person.name, &person.fav_color);
        }
    }
}
