use std::{
    sync::Arc,
    sync::Mutex,
    sync::mpsc,
    thread::{self, JoinHandle},
};

struct Worker {
    id: usize,
    jh: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let jh = thread::spawn(move || {
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a msg.", id);
                match msg {
                    Message::NewJob(job) => job(),
                    Message::Terminate => break,
                }
            }
        });
        Worker { id, jh: Some(jh) }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// 创建线程池
    ///
    /// 线程池中线程的数量
    ///
    /// #Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: u32) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::new();
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..size {
            workers.push(Worker::new(i.try_into().unwrap(), Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.jh.take().unwrap().join().unwrap();
        }
    }
}
