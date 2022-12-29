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
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Result<Worker, std::io::Error> {
        let thread = Builder::new().spawn(move || loop {
            let job = match receiver.lock() {
                Ok(guard) => match guard.recv() {
                    Ok(job) => job,
                    Err(error) => {
                        //TODO: Add logic to handle error instead of log it.
                        eprintln!("An error occurred in the job: {}", error);
                        return;
                    }
                },
                Err(error) => {
                    //TODO: Add logic to handle error instead of log it.
                    eprintln!("An error occurred in the guard: {}", error);

                    return;
                }
            };
            println!("Worker {id} got a job; executing.");

            job();
        })?;

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.as_ref().unwrap().send(job) {
            Ok(possibly_received) => possibly_received,
            Err(error) => eprintln!(
                "An error ocurred, the receiver don't get the message  {:?}",
                error
            ),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // Iterate through all of the workers in the thread-pool
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // Check if the worker has a thread that it is running
            if let Some(thread) = worker.thread.take() {
                // If the worker has a thread, finish the work and try to shut down.
                match thread.join() {
                    Ok(t) => t,
                    Err(error) => eprintln!(
                        "An error ocurred shutting down the worker, error:  {:?}",
                        error
                    ),
                }
            }
        }
    }
}
