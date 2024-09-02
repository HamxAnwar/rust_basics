fn main() {
    // To specify an array, we can do the following:
    let an_array = [1, 2, 3];

    // Now to specify a vector:

    let mut a_vector: Vec<i32> = Vec::new(); //Initialization
    a_vector.push(4);
    a_vector.push(5);
    a_vector.push(6);

    // If we want to initialize and assign values to a vector at the same time, we cando it using the macro as follows:
    let mut second_vector = vec![1, 2, 3];
    // println!("The second element of the above vector is: {:#?}", &second_vector[1]);

    //If we give an index which is out of bound, the vector would through a runtime error and not a compile time error which would be a case for the same when using arrays. The reason is that the size of an array is known while the size of a vector is unknown. Moreover, when using a vector instead of an array, you agree to expect that the program can break at runtime if a wrong index is passed. If we want to tackle that, we can use the built-in .get method as follows:

    let second = &second_vector[1];
    // println!("{:#?}", second_vector.get(10));
    match second_vector.get(10) {
        Some(second) => println!(
            "This is the second element using the .get method: {}",
            second
        ),
        None => println!("There is no second element."),
    }

    for i in &second_vector {       // Here we can also use a mut reference if we want to modify the vector.
        println!("{}", i);
    }

    for i in &mut second_vector {
        *i += 50;                   // The de-reference operator.
    }

    for i in &second_vector {
        println!("{}", i);
    }

    // What if we want to define a vector that can contain different types of elements. We can use
    // a vector consisting of enums for that as follows:

    enum SpreadsheetCells {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCells::Int(32),
        SpreadsheetCells::Float(1.35),
        SpreadsheetCells::Text(String::from("A text...")),
    ];

    // The only cath in the above is that we have to use match statement to access the elements of
    // an enum vector.
}
