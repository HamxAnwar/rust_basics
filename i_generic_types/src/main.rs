fn main() {
    let number_list = vec![10, 20,30, 40, 50, 60];
    let number_list_1 = vec![10, 20,30, 40, 50, 60];
    let number_list_2 = vec![1,2,4,5,3,2,7,9,2];
    let char_list = vec!["a", "b", "c", "d", "e", "z", "j"];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    // What if we want to find the largest number in multiple vectors. The above code can be managed for that by converting the logic into a function.

    let largest_v1 = find_largest(number_list_1);
    let largest_v2 = find_largest(number_list_2);
    let number_list_1 = vec![10, 20,30, 40, 50, 60];
    let largest_num = find_largest_with_generics(number_list_1);
    let largest_char = find_largest_with_generics(char_list);
    eprintln!("largest in Vec 1 = {:#?}, largest in Vec 2 = {:#?}", largest_v1, largest_v2);
    eprintln!("largest in number = {:#?}, largest in character = {:#?}", largest_num, largest_char);

    // But what if we have an array of integers while the other array is of characters. The input types to our function for both the arrays will be different. We can use generics to specify a generic type for the input.

}

fn find_largest(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest_with_generics<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}