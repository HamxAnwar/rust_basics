// Iterator allows us to iterate over a sequence of elements regardless of how the elements are stored, i.e. in an array, a vector, hashmap, etc.
// Iterating over a vector is easy: Start from index 0, 1, 2 and so on.
// Iterating over hasmaps or other data structures is not as obvious.

/*
fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // This is the iterator which is lazy by default so it will not do anything by default.
    for element in v1_iter {
        // Lets iterate over our vector.
        println!("{element}");
    }

    // Remember that the iterator return immutable references.
    // To get mutable references, it will be v1.iter_mut() and v1.into_iter() if we want our own types.
}
*/

// Iterators have two broad categories of iterator methods.

/*
1. Adapters: Take one iterator and return another iterator.

fn main() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{v2:?}");
}
*/

/*
2. Consumers: Take one iterator and return another type such as int, string, float, etc. e.g.

fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{total}");
}
*/

/*
// Iterators with closures that capture the environment.

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == size).collect() // Since it returns our own type, we use into_iter.
}

fn main() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneakers"),
        },
        Shoe {
            size: 14,
            style: String::from("sandals"),
        },
        Shoe {
            size: 13,
            style: String::from("boots"),
        },
        Shoe {
            size: 10,
            style: String::from("q-chappal"),
        },
        Shoe {
            size: 20,
            style: String::from("sneakers"),
        },
    ];
    println!("{:#?}", shoe_in_my_size(shoes, 10));
}
*/

// But we can use our own iterators too. it can be done as follows:
// For demonstration purposes, our iterator will only count from 1 to 5.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_net_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// The std library introduces alot of other methods for the iterators so lets right below test for those methods:

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// In the above code:
// We are creating a new counter and calling the .zip method on it.
// The zip method will take two iterators and zips them into one iterator containing pairs of values.
// The first iterator will the iterator on which the method is called on and the second iterator is passed into it.
// In th second parameter, we are again creating a counter and calling skip method which is an adaptive method so it will return another iterator. It will skip the first n elements; n = number passed into the method.
// Next we call map which will take the iterator, call a closure on it taking two parameters, a and b. It will then multiply the parameters.
// Then we filter the items that are divisible by three.
// Sum them and check the test.

fn main() {}
