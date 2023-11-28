use std::{sync::{Mutex, Arc}, thread, time::Duration};

fn mutex_in_single_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_counter() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn try_deadlock() {
    let r1 = Arc::new(Mutex::new(0));
    let r2 = Arc::new(Mutex::new(1));

    let r1h1 = Arc::clone(&r1);
    let r2h1 = Arc::clone(&r2);
    let h1 = thread::spawn(move || {
        let mut num1 = r1h1.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        println!("[t1]: try lock r2h1");
        let mut num2 = r2h1.lock().unwrap();
        println!("[t1]: done");


        *num1 += 1;
        *num2 += 1;
    });

    let r1h2 = Arc::clone(&r1);
    let r2h2 = Arc::clone(&r2);
    let h2 = thread::spawn(move || {
        let mut num2 = r2h2.lock().unwrap();
        thread::sleep(Duration::from_secs(1));
        println!("[t2]: try lock r1h2");
        let mut num1 = r1h2.lock().unwrap();
        println!("[t2]: done");

        *num1 += 1;
        *num2 += 1;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!("r1: {}, r2: {}", *r1.lock().unwrap(), *r2.lock().unwrap());
}

pub fn run() {
    mutex_in_single_thread();
    mutex_counter();
    // !!! will cause deadlock
    // try_deadlock();
}