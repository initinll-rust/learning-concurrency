use std::{time::Duration, thread};

pub fn create_new_threads() {
    println!("\ncreate_new_threads");
    // create new threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // wait for threads to complete
    handle.join().unwrap();
}