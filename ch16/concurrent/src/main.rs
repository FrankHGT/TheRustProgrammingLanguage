use std::{thread, time::Duration};

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

fn main() {
    create_thread();

    pass_data_to_thread();
}
