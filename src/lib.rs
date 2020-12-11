//! # Thread pool from the Tutorial
//!
//! The thread pool described within the tutorial for the web server.
//! It pretty cool

use std::thread;
use std::sync::{mpsc, Arc, Mutex};

/// Wraps upt the needed traits into one type.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Allows us more verbage with the workes
enum Message {
    NewJob(Job),
    Terminate,
}

/// # Main Struct
///
/// Holds the pool of workers as well as the channed used to talk to the workers.
/// The constructor returns the result.
/// Can be initialized with the include constructer:
/// ```
/// use light_sign::ThreadPool;
/// let pool = match ThreadPool::new(4) {
///     Ok(t) => t,
///     Err(_) => panic!("It's too big"),
/// };
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

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

        let mut workers = Vec::with_capacity(size);

        // make all the theads
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }
    /// Send work to a thread pool
    ///
    /// Send a message to all workers telling them to die.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// #Destroys a threadpopl
    ///
    /// Send a message to all workers telling them to die.
    fn drop(&mut self) {
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);
            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message: Message = receiver
                .lock()
                .unwrap() // lock the channel mutex
                .recv()
                .unwrap(); // get the job from the channel

            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
