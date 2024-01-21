use core::fmt;
use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// A pool of worker threads that handles the spawning 
/// and allocation of jobs to worker threads.
///
/// Send closures to executed with ThreadPool.execute()
///
/// # Example: Web server
/// ```
/// fn main() {
///
///     let listener = TcpListener::bind(format!("{127.0.0.1:7878")).unwrap();
///     let pool = ThreadPool::new(4);
///
///     for stream in listener.incoming().take(5) {
///         let stream = stream.unwrap();
///         pool.execute(|| {
///             handle_connection(stream);
///         });
///     }
///
///     println!("Server shutting down");
/// }
///
/// fn handle_connection(mut stream: TcpStream) {}
/// ```
///
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// size is the number of threads
    ///
    /// # Panics
    ///
    /// Will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Build a new ThreadPool
    ///
    /// size is the number of threads
    ///
    /// Results in PoolCreationError if the size is zero
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::new(
                "Thread pool cannot be initilised with 0 threads",
            ));
        }

        Ok(ThreadPool::new(size))
    }

    /// Send onetime closure to thread pool to be executed
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

/// Elegant worker shutdown
///
/// Makes sure that the channel senders are dropped and that jobs are finished
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // TODO
        // thread::spawn can panic, consider using thread::builder
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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

type Job = Box<dyn FnOnce() + Send + 'static>;

/// Returned when there is an error in ThreadPool creation
///
/// Probably because of an invalid number of threads
#[derive(Debug)]
pub struct PoolCreationError {
    details: String,
}

impl PoolCreationError {
    fn new(message: &str) -> PoolCreationError {
        PoolCreationError {
            details: message.to_string(),
        }
    }
}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for PoolCreationError {
    fn description(&self) -> &str {
        &self.details
    }
}
