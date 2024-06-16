## Concurrency and Parallelism with Rust

This repository contains a collection of Rust projects demonstrating some concepts in concurrency and parallelism. Each project is organized as a separate 
package within a Cargo workspace.

### Getting Started

To get started with these projects, ensure you have [Rust](https://www.rust-lang.org/) installed on your machine. You can install Rust using `rustup` run the 
following command in your terminal, then follow the onscreen instructions.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Executing the code

under the root workspace execute the cargo command below

```shell
cargo run --bin <project-name> # e.g., _00-new-thread-with-spawn
```

or, inside the project directory `cargo run`

### Projects overview

Here you can find an overview about the current codes.  

#### _00-new-thread-with-spawn

In this project we create multiple threads using the **thread::spawn** function from the standard library. Each thread print "hey, {thread_identifier} either 
from main or spawn thread". We also learn how to keep threads alive until all of they are all executed.

```rust
let handle = thread::spawn(move || {
    println!("Hey, from thread {}", i);
});
```

#### _01-sum-array-in-parallel

This project demonstrates how to sum all the elements of an array in parallel using multiple threads. It also use data decomposition by dividing the vector into 
chunks.

```rust
for chunck in vector.chunks(chunk_vec_size) {
    let chunk = chunck.to_owned();
    vec_threads.push(thread::spawn(move || -> i32 {
        chunk.iter().sum()
    }));
}
```

#### _02-thread-safety-arc-mut

This project implements a thread-safe counter using a `Mutex` to ensure safe increments among multiple threads and `Arc` used to share ownership of the 
`Mutex`. Mutex ensures that multiple threads can safely increment the counter without causing race conditions.   

See [Mutex](https://doc.rust-lang.org/std/sync/struct.Mutex.html)  

* Mutex allows only one thread to access the counter at a time;  
* Each thread gets a clone of the Arc pointer, incrementing the reference count;  
* Before accessing the counter, each thread locks the Mutex using `lock()`. This ensures that only one thread can modify the counter at a time;  
* After all threads have finished, the counter result is printed.   

#### _03-producer-consumer-buffer

The producer-consumer problem is a classic example of a multi-threading problem where a fixed-size buffer is shared between producer threads that generate data 
and consumer threads that process data.

## License

This project is licensed under the MIT License.
