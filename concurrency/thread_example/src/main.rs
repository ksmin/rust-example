fn thread_spawn_join_move() {
    use std::{thread, time::Duration};

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

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn channel() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

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

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex() {
    use std::thread;
    use std::sync::{Arc, Mutex};

    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));

    for idx in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut val = counter.lock().unwrap();
            *val += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Count: {}", counter.lock().unwrap());
}

fn main() {
    thread_spawn_join_move();
    mutex();
    channel();
}
