use std::thread;

pub fn move_ownership() {
    println!("\nmove_ownership");
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}