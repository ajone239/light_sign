use std::thread;
use std::sync::{mpsc, Arc, Mutex,};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the nube of threads in the pool.
    ///
    /// # Panics
    ///
    /// The new function panics with a zero size.
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size > 100 {
            return Err("Too big");
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers= Vec::with_capacity(size);

        // make all the theads
        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool{ workers, sender })

    }
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move ||  loop {
            let job = receiver.lock().unwrap() // lock the channel mutex
                .recv().unwrap(); // get the job from the channel

            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}
