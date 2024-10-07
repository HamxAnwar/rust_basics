// Rust handles concurrent programming efficiently.
// Concurrent programming is when different parts of the program executes independently and parallel programming is when different parts of the program executes at the same time.
// Both are very closely tied. So usually they are used together in the same context.
// We use threads for concurrency.
// Increases performance but also complexity.
// Challenges:
// - Raise conditions: Threads access data or resources in an inconsistent manner.
// - Deadlocks: Two threads such that thread A is waiting for the resources that the other thread has and vice versa. So both the threads have to wait indefinitely.
// - Execution order is non-deterministic.
// Two types of threads:
// - One to one threads, a.k.a. os threads, native threads, system threads, etc.
//      - Provided by various OS as API to create new threads.
//      - Creating new threads would map it to an OS thread.
// - Green threads, a.k.a. user threads or program threads, etc.
//      - Implementations of OS threads itself.
//      - Donot have one to one mapping.
//      - So we can have 20 green threads that only map to 5 OS threads.
//      - This model is also called m to n model.
// Each model has its own merits and demerits.
// In rust, the tradeoff is the runtime support. Because each thread should be included in each binary, resulting in greater runtime. Rust aims for smaller runtimes.
// This introduces the tradeoff which is less features support, therefore lowering the runtime.
// Green threads have higher runtime so rust only supports one to one threads in its std library.

// Lets get threads into code.

/*
use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi! number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi! number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/

// When we run the above, we can see that multiple runs of the code will result in multiple outputs due to non-deterministic execution.
// Also as the main thread is closed, the code stops and the spawn thread is not executed completely.
// Lets see how to modify the code to finish the spawn code.

/*
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        // we need to use a JoinHandle type.
        for i in 1..10 {
            println!("hi! number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi! number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // Now in the end we call the handle.join to join the threads and let them complete.
                           // Calling the join function lets the function, here the main function, to not end and calls the variable on which join is called to complete.
}
*/

// Remember if we call the handle.join before the for loop as below, it will wait for the spawn thread to complete and then transfer to the main thread.

/*
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi! number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi! number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
*/

// Now lets see how to use move with threads.
// Until now, when we spawn the thread, it doesn't depend on any variables outside the thread.

use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);     // We cannot call the drop function as v is moved into the closure and it can't access it.
    handle.join().unwrap();
}
