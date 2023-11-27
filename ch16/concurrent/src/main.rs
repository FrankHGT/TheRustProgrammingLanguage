use std::{thread, time::Duration, sync::mpsc};

fn pass_data_to_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    // we can't borrow v in spawn(), cause execution order
    // between thread is unpredictable
    // drop(v);

    handle.join().unwrap();
}

fn create_thread() {
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

    // block current thread(join call thread)
    // until handle thread end
    handle.join().unwrap();
}

fn message_passing() {
    // mpsc for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
        // moved in to channel already
        // for val in vals {
        //     println!("val in vals {}", val);
        // }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
        // moved in to channel already
        // for val in vals {
        //     println!("val in vals {}", val);
        // }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    create_thread();

    pass_data_to_thread();

    message_passing();
}
