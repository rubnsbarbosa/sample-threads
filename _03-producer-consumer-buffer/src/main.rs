/*
 * The producer-consumer problem is a well known example of a multi-threading 
 * where a fixed-size buffer is shared between producer threads that generate 
 * data and consumer threads that consume data. 
 *
 * The producers can't produce when the buffer is full
 * The consumers can't consume when the buffer is empty
 *
 * */
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};

const N: i32 = 20;

#[derive(Debug, Default)]
struct Buffer {
    data: Mutex<Option<i32>>,
    data_condv: Condvar,
}

impl Buffer {
    fn insert(&self, value: i32) {
        let mut lock = self.data.lock().expect("not locked");
        while lock.is_some() {
            lock = self.data_condv.wait(lock).expect("not waiting lock");
        }
        *lock = Some(value);
        self.data_condv.notify_one();
    }

    fn remove(&self) -> i32 {
        let mut lock = self.data.lock().expect("not lock");
        while lock.is_none() {
            lock = self.data_condv.wait(lock).expect("not waiting lock");
        }
        let value = lock.take().unwrap();
        self.data_condv.notify_one();
        return value;
    }
} 

fn producer(buffer: &Buffer) {
    for i in 0..N {
        println!("producer thread id: {}", i);
        buffer.insert(i);

        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(buffer: &Buffer) {
    for i in 0..N {
        let val = buffer.remove();
        println!("consumer thread id: {} - consumed: {}", i, val);

        thread::sleep(Duration::from_millis(125));
    }
}

fn main() {
    let buffer = Arc::new(Buffer::default());

    let buffer_producer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        producer(&buffer_producer);
    });

    let buffer_consumer = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        consumer(&buffer_consumer);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

