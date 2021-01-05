//! # Thread pool from the Tutorial
//!
//! The thread pool described within the tutorial for the web server.
//! For it's use here is has remained largely unchanged from what I wrote.

#![allow(dead_code)]

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Wraps upt the needed traits into one type.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Allows us more verbage with the workes
enum Message {
    /// Contains work for a thread to run
    NewJob(Job),
    /// Signals all the threads to stop working and shut down
    Terminate,
}

/// # Main Struct
///
/// Holds the pool of workers as well as the channed used to talk to the workers.
/// The constructor returns the result.
/// Can be initialized with the include constructer:
/// ```
/// mod thread_pool;
///
/// use crate::thread_pool::ThreadPool;
/// let pool = ThreadPool::new(4);
///
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the nube of threads in the pool.
    ///
    /// # Examples
    /// ```
    /// let pool = ThreadPool::new(THREAD_COUNT);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // make all the theads
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    /// Sends work to a thread pool.
    /// f must be of the type *F: FnOnce() + Send + 'static*.
    ///
    /// # Examples
    /// ```
    /// let pool = ThreadPool::new(THREAD_COUNT);
    /// let int_ref = Arc::new(Mutex::new(0));
    ///
    /// for i in 0..5 {
    ///     let ref_clone = Arc::clone(&int_ref);
    ///     pool.execute(|| {
    ///         let shared = ref_clone.lock().unwrap();
    ///         shared += 1;
    ///     });
    /// }
    /// ```
    ///
    /// # Panics
    /// If the mpsc channel send fails.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Destroys a threadpopl.
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

// Helper Struct
// Holds a thread and an id for tracking
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
