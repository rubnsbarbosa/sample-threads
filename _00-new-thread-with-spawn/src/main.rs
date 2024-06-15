use std::thread;
use std::time::Duration;
use crate::thread::JoinHandle;

const MAX_NUM_THREADS: i16 = 10;
const MAX_NUM_THREADS_IN_MAIN: i16 = 5;

fn shut_down_spawned_threads() {
    // the spawned threads' gonna be killed when the main thread finish their run
    // as their max size is half of the spawned one
    thread::spawn(|| { 
        for i in 1..MAX_NUM_THREADS {
            println!("hey, {i} from spawn thread");
            thread::sleep(Duration::from_millis(1));
        }
    });
}

fn avoid_shut_down_spawned_threads() -> JoinHandle<()> {
    let handle = thread::spawn(|| {
        for i in 1..MAX_NUM_THREADS {
            println!("hey, {i} from spawn thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    return handle;
}

// Now the two threads continue alternating, but the main thread waits because of the call to handle.join() 
// and does not end until the spawned thread is finished. BE AWARE, if we move the handle.join() to
// between spawn and main thread, the behaviour will be different. It'll finish all spawn first!
fn main() {
    //shut_down_spawned_threads();
    let handle = avoid_shut_down_spawned_threads();
    
    for i in 1..MAX_NUM_THREADS_IN_MAIN {
        println!("hey, {i} from main thread!!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

