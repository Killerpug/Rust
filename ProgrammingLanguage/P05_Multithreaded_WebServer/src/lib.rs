use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

// a thread pool has workers(thread managers) and code to split between those workers
pub struct ThreadPool {
    workers: Vec<Worker>,   // A worker is in charge of sending code from threadPool to thread
    sender: mpsc::Sender<Message>, //sender has the code to send to a worker
}


type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    // --snip--
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0); // valid size
        //create channel to pass http requests
        let (sender, receiver) = mpsc::channel();
        // allow to pass receiver to multiple Worker instances. We want to share single receiver across multiple threads
        // also add Mutex to control access
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); // space for storing the threads
        for id in 0..size {
            //attach id and channel receiver to the worker (thread manager)
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }

    // takes a closure and gives it to a thread in the pool to run
    pub fn execute<F>(&self, f: F)
    where
    // closure FnOnce, because we will pass the argument to spawn
    // 'static because we Send() the closure from one thread to another and we dont know how long the thread take to execute.
        F: FnOnce() + Send + 'static,   
    {
        let job = Box::new(f);

        // sends job to worker through the message channel 
        self.sender.send(Message::NewJob(job)).unwrap();    //this will be received by worker receiver side
    }
}

// A worker is thread manager
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,     // assigned thread
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // lock receiver and get http request to run
            //recv() blocks so if there is no job, the current thread wait until a job is available.
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();  // runs job
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// cleanup
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}