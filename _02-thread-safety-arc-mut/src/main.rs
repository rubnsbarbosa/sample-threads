/*
 * Thread safety is crucial in concurrent applications to prevent data races
 * Implementing thread safety in Rust involve using sync primitive Arc & Mutex
 *
 * Arc (Atomically Reference Counted) - it shares ownership between threads
 * Mutex - it will block threads waiting for the lock to become available, i.e.,
 * it will exclude those who's waiting, allowing only one thread to access the counter at a time.
 *
 * This code will increment a counter with multiple threads without causing race conditions
 * */
use std::sync::{Arc, Mutex};
use std::thread;

const N: usize = 20;

fn main() {
    // here we are using an Arc to share memory among threads, 
    // and the data inside the Arc is protected with a mutex.
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..N {
        // clone the Arc to share the counter between threads
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // lock the clone mutex before accessing the counter
            let mut value = counter_clone.lock().unwrap();
            // increment the counter
            *value = *value + 1;
        });
        
        handles.push(handle);
    }

    // waiting all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Count result = {}", *counter.lock().unwrap());
}

