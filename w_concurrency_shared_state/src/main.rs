// Here, we will talk about transfering data using shared state.
// Data sharing between thread is a one way data flow.
// The shared state enables us to have a memory piece which multiple threads can read and write to.
// Mutex: Mutual Exclusion: We have a data and only one thread can only access that data at a given time.
// When one thread needs a data, it will lock itself there and keeps a track of which thread, the data belong to.
// Once a thread acquires a piece of data, no other thread can access it until it is released/unlocked.
// Mutex are hard:
//      - Acquire a lock before getting access to data.
//      - Release the lock when done with the data, so other threads can use it.

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
