// Smart pointers also implement a drop trait.
// A drop trait could be implemented on any type.
// It allows us to customize what happens when a value goes out of scope.
// It is almost always used for smart pointers.
// It makes the cleaning up smart pointers automatically rather then manually cleaning the memory up.

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my drop trait"),
    };
    // drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomerSmartPointers created");
}
// Run the above code to see what happens.
// It will execute the drop at the end of main after which it will be out of scope.
// Also remember that the dropping occurs in the reverse order of creation. So first, d will be dropped and then c will be dropped.
// Also if we want to early drop a pointer, rust doesn't let us do:
//              c.drop.
// Instead we have to use the builtin drop function as:
//              drop(c);
// This will drop the pointer before the ending of the scope.
// We can uncomment above in the code to check it out.
