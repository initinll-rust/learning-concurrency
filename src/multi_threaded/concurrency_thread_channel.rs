use std::{sync::mpsc, thread, time::Duration};

pub fn message_passing_between_threads() {
    println!("\nmessage_passing_between_threads");
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    // Thread 1 - Transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Thread 2 - Transmitter
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Main thread - Receiver
    for received in rx {
        println!("Got: {}", received);
    }
}