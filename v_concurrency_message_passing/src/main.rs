// Lets learn about using messages to pass data between threads.
// We can use channels which is inside the std library for this.
// A channel will have two parts:
//  - Transmitter: Upstream location
//  - Reciever: Downstream location
// The channel is closed when either the transmitter or the receiver is dropped.

use std::{sync::mpsc, thread}; // Multi Producer Single Consumer
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Hi!");
        tx.send(msg).unwrap();
    });

    let recieved = rx.recv().unwrap(); // recv will stop the code and wait for the message. If we want to just try and let the code run parallelly, then we can use try_recv.
    println!("Got: {}", recieved);
}

// Remember, when a value is passed from one thread to another, it transfers its ownership and can't be reused.
// Also if we have multiple threads, we cannot use tx for both threads. So we need to make another variable as "let tx2 = tx.clone()"
