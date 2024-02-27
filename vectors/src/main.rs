// Vectors are used to store multiple pieces of data.
// Data must be of same time.
// Can add, remove and traverse the entries of vectors.
// Two ways to define a vector:
/*

-- let my_numbers = vec![1, 2, 3];
-- let mut my_numbers = Vec::new();

*/
/*

my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);
my_numbers.pop();   // Removes the last item.
my_numbers.len();   // Gives 2 as output as removed the last item.

let two = my_numbers[1]

*/

/*

let my_numbers = vec![1, 2, 3];
for num in my_numbers {
    println!("{:?}", num);
}

*/

fn print_numbers(vec: &Vec<i32>) {
    for num in vec {
        if num == &30 {
            println!("thirty");
        }
        else {
            println!("{:?}", num);
        }
    }
}

fn main() {
    let numbers = vec![10, 20, 30, 40];
    print_numbers(&numbers);
    let length = numbers.len();
    println!("Length of the array: {:?}", length)
}
