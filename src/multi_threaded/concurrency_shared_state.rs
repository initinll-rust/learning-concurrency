use std::{sync::{Mutex, Arc}, thread};

pub fn try_mutex_single_thread() {
    println!("\ntry_mutex");
    let m =  Mutex::new(0);

    {
        let mut num = m.lock().unwrap();
        *num = 5;
    }

    println!("m = {:?}", m);
}

pub fn share_mutex_multi_thread() {
    // Arc - Atomic reference counter
    // mutex - mutually exclusive interior mutability
    // Cell<T>/RefCell<T>/Rc<T> (single thread) similar to Mutex<T>/Arc<T> (multi thread)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}