use std::error;
use std::thread;

/// An error that can occur when creating a `ThreadPool`.
#[derive(Debug)]
pub enum PoolCreationError {
    /// The size of the thread pool must be greater than zero.
    InvalidSize,
}

impl error::Error for PoolCreationError {}

impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid thread pool size")
    }
}

pub struct ThreadPool;

impl ThreadPool {
    /// Creates a new `ThreadPool` with the specified number of worker threads.
    /// # Panics
    /// This function will panic if `size` is zero.

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

/// Builds a new `ThreadPool` with the specified size.
/// # Errors
/// This function will return an error if the size of the thread pool is zero.
/// # Examples
/// ```
/// use threadpool::{ThreadPool, build};
///
/// let pool = build(4).unwrap();
/// ```
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    if size == 0 {
        return Err(PoolCreationError::InvalidSize);
    }

    Ok(ThreadPool::new(size))
}
