use std::io::Error;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, Builder, JoinHandle};
/// An error that can occur when creating a `ThreadPool`.
#[derive(Debug)]
pub enum PoolCreationError {
    /// The size of the thread pool must be greater than zero.
    InvalidSize,
}

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid thread pool size")
    }
}

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, std::io::Error> {
        let thread = Builder::new().spawn(|| {
            receiver;
        })?;

        Ok(Worker { id, thread })
    }
}

struct Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Creates a new `ThreadPool` with the specified number of worker threads.
    /// # Errors
    /// This function will return an error if the size of the thread pool is zero, or if there is an error creating a worker thread.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size <= 0 {
            return Err(PoolCreationError::InvalidSize);
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker = match Worker::new(id, Arc::clone(&receiver)) {
                Ok(worker) => worker,
                Err(_) => {
                    return Err(PoolCreationError::InvalidSize);
                }
            };
            workers.push(worker);
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}
