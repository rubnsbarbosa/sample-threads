use std::thread;

const VEC_SIZE: i32 = 100;
const VEC_SPLT: usize = 10;
const MAX_MAIN_THREAD: i32 = 5;

fn sum_parallel_vector(vector: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    let mut vec_threads = vec![];
    let chunk_vec_size = vector.len() / VEC_SPLT;    
   

    for chunck in vector.chunks(chunk_vec_size) {
        let chunk = chunck.to_owned();
        vec_threads.push(thread::spawn(move || -> i32 {
            chunk.iter().sum()
        }));
    }

    for thread in vec_threads {
        sum += thread.join().unwrap()
    }


    return sum;
}

fn main() {
    // create a vector within elements = 1
    let mut v = vec![];
    for _ in 0..VEC_SIZE {
        v.push(1);
    }

    for i in 0..MAX_MAIN_THREAD {
        let sum = sum_parallel_vector(&v);
        println!("hey, thread {} - the sum = {}", i, sum);
    }
}

