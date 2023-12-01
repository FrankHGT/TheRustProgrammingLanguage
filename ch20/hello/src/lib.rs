use std::{thread, sync::{mpsc::{Sender, Receiver, self}, Arc, Mutex}};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // use Arc<Mutex<T>>, you can pass any data structure between thread
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        let runloop = move || {
            loop {

                // 2. this will work cause MutexGuard<T> will out of scope
                // when we get job
                let msg = receiver.lock().unwrap().recv().unwrap();

                match msg {
                    Message::NewJob(job) => {
                        // 1. it won't work, cause receiver will be MutexGuard<T>,
                        // it will block other thread's lock, until this loop over
                        // let receiver = receiver.lock().unwrap();
                        // let job = receiver.recv().unwrap();

                        println!("Worker {} got a job; executing.", id);

                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} Received terminate message, exiting.", id);
                        break;
                    }
                }

            }
        };

        Worker {
            id,
            thread: Some(thread::spawn(runloop)),
        }
    }
}

pub struct ThreadPool {
    sender: Sender<Message>,
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 创建线程池
    /// 
    /// 线程池中线程的数量
    /// 
    /// # Panics
    /// 
    /// `new`函数会在size为0时触发panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        // with_capacity只会预分配空间，不会占用真正的空间
        // 类似于c++ vector中capacity和size的关系
        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(rx));

        for id in 0..size {
            // TODO
            // receiver.clone()
            let worker = Worker::new(id, Arc::clone(&receiver));

            workers.push(worker);
        }

        ThreadPool {
            sender: tx,
            workers,
        }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }

    pub fn quit(&self) {
        for _ in 0..self.workers.len() {
            self.sender.send(Message::Terminate).unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.quit();

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            // take() function will take v's ownership in Some(v)
            // and replace None with it.
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

// this is not implement Send trait, so compile will error
// when we try to move it to another thread
// pub struct Job {
//     closure: Box<dyn FnOnce()>,
// }

// impl Job {
//     pub fn new(closure: Box<dyn FnOnce()>) -> Job {
//         Job {
//             closure
//         }
//     }

//     pub fn run(&self) {
//         (*self.closure)()
//     }
// }