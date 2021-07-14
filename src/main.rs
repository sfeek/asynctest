use std::sync::{Arc, RwLock};
use std::{thread, time};

fn main() {
    // A variable to hold a Mutex shared memory location
    let c = Arc::new(RwLock::new(0));

    // Cloned variable used in the sub thread
    let thread_c1 = Arc::clone(&c);

    // Cloned variable used in the sub thread
    let thread_c2 = Arc::clone(&c);

    // Create a thread to do a long running job
    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            cnt += 1;
            println!("Still going in 1... {}", cnt);
            thread::sleep(time::Duration::from_millis(100));
            if *thread_c1.read().unwrap() == 1 {
                break;
            }
        }
    });

    // Create a 2nd thread to do a long running job
    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            cnt += 1;
            println!("Still going in 2... {}", cnt);
            thread::sleep(time::Duration::from_millis(100));
            if *thread_c2.read().unwrap() == 1 {
                break;
            }
        }
    });

    // Hang Out in local thread for a while
    thread::sleep(time::Duration::from_millis(2000));

    // Signal for threads to stop
    *c.write().unwrap() = 1;
}